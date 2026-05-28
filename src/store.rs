use std::sync::{Mutex, MutexGuard};

use rusqlite::{Connection, params};

use crate::embedding::{decode_vector, encode_vector};
use crate::models::{Chunk, Document};

pub struct SqliteStore {
    connection: Mutex<Connection>,
}

impl SqliteStore {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: Mutex::new(connection),
        }
    }

    pub fn init(&self) -> rusqlite::Result<()> {
        let connection = self.connection()?;
        connection.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                source TEXT NOT NULL,
                text TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS chunks (
                id TEXT PRIMARY KEY,
                document_id TEXT NOT NULL,
                chunk_index INTEGER NOT NULL,
                text TEXT NOT NULL,
                source TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(document_id) REFERENCES documents(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS embeddings (
                chunk_id TEXT PRIMARY KEY,
                model TEXT NOT NULL,
                dimensions INTEGER NOT NULL,
                vector BLOB NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(chunk_id) REFERENCES chunks(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS rag_queries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                question TEXT NOT NULL,
                answer TEXT NOT NULL,
                citations_json TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )?;
        Ok(())
    }

    pub fn save_document(&self, document: &Document) -> rusqlite::Result<()> {
        let connection = self.connection()?;
        connection.execute(
            r#"
            INSERT INTO documents (id, title, source, text)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                source = excluded.source,
                text = excluded.text
            "#,
            params![document.id, document.title, document.source, document.text],
        )?;
        Ok(())
    }

    pub fn save_chunk_with_embedding(
        &self,
        chunk: &Chunk,
        vector: &[f32],
        model: &str,
    ) -> rusqlite::Result<()> {
        let connection = self.connection()?;
        connection.execute(
            r#"
            INSERT INTO chunks (id, document_id, chunk_index, text, source)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                text = excluded.text,
                source = excluded.source
            "#,
            params![
                chunk.id,
                chunk.document_id,
                chunk.chunk_index as i64,
                chunk.text,
                chunk.source
            ],
        )?;
        connection.execute(
            r#"
            INSERT INTO embeddings (chunk_id, model, dimensions, vector)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(chunk_id) DO UPDATE SET
                model = excluded.model,
                dimensions = excluded.dimensions,
                vector = excluded.vector
            "#,
            params![chunk.id, model, vector.len() as i64, encode_vector(vector)],
        )?;
        Ok(())
    }

    pub fn load_chunks_with_embeddings(&self) -> rusqlite::Result<Vec<(Chunk, Vec<f32>)>> {
        let connection = self.connection()?;
        let mut statement = connection.prepare(
            r#"
            SELECT c.id, c.document_id, c.chunk_index, c.text, c.source, e.vector
            FROM chunks c
            JOIN embeddings e ON e.chunk_id = c.id
            "#,
        )?;

        let rows = statement.query_map([], |row| {
            let bytes: Vec<u8> = row.get(5)?;
            Ok((
                Chunk {
                    id: row.get(0)?,
                    document_id: row.get(1)?,
                    chunk_index: row.get::<_, i64>(2)? as usize,
                    text: row.get(3)?,
                    source: row.get(4)?,
                },
                decode_vector(&bytes),
            ))
        })?;

        rows.collect()
    }

    pub fn record_answer(
        &self,
        question: &str,
        answer: &str,
        citations_json: &str,
    ) -> rusqlite::Result<()> {
        let connection = self.connection()?;
        connection.execute(
            "INSERT INTO rag_queries (question, answer, citations_json) VALUES (?1, ?2, ?3)",
            params![question, answer, citations_json],
        )?;
        Ok(())
    }

    fn connection(&self) -> rusqlite::Result<MutexGuard<'_, Connection>> {
        self.connection.lock().map_err(|_| {
            rusqlite::Error::InvalidParameterName("sqlite connection lock poisoned".to_string())
        })
    }
}
