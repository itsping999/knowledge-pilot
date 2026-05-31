use crate::models::{Chunk, Document};

pub fn document_title_from_text(text: &str) -> Option<String> {
    text.lines()
        .find_map(|line| parse_markdown_heading(line.trim()))
        .filter(|title| !title.trim().is_empty())
}

pub fn chunk_document(document: &Document, chunk_size: usize, chunk_overlap: usize) -> Vec<Chunk> {
    let text = document.text.trim();
    if text.is_empty() || chunk_size == 0 {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = text.chars().collect();
    let headings = markdown_headings(text);
    let overlap = chunk_overlap.min(chunk_size.saturating_sub(1));

    while start < chars.len() {
        let hard_end = usize::min(start + chunk_size, chars.len());
        let end = if hard_end < chars.len() {
            preferred_chunk_end(&chars, start, hard_end).unwrap_or(hard_end)
        } else {
            hard_end
        };
        let chunk_text: String = chars[start..end].iter().collect();
        let chunk_text = chunk_text.trim().to_string();
        if !chunk_text.is_empty() {
            let chunk_index = chunks.len();
            chunks.push(Chunk {
                id: format!("{}:{}", document.id, chunk_index),
                document_id: document.id.clone(),
                chunk_index,
                document_title: document.title.clone(),
                section: section_for_position(&headings, start)
                    .unwrap_or_else(|| document.title.clone()),
                text: chunk_text,
                source: document.source.clone(),
                access_scope: document.access_scope.clone(),
            });
        }
        if end == chars.len() {
            break;
        }
        let rough_next_start = end.saturating_sub(overlap);
        let next_start = preferred_chunk_start(&chars, rough_next_start, end)
            .filter(|candidate| *candidate > start)
            .unwrap_or(rough_next_start);
        start = if next_start > start { next_start } else { end };
    }

    chunks
}

fn preferred_chunk_end(chars: &[char], start: usize, hard_end: usize) -> Option<usize> {
    let min_end = start + ((hard_end - start) / 2).max(1);
    for index in (start..hard_end).rev() {
        if index + 1 < min_end {
            break;
        }
        if is_chunk_boundary(chars[index]) {
            return Some(index + 1);
        }
    }
    None
}

fn preferred_chunk_start(chars: &[char], rough_start: usize, previous_end: usize) -> Option<usize> {
    for (index, ch) in chars
        .iter()
        .enumerate()
        .take(previous_end)
        .skip(rough_start)
    {
        if is_chunk_boundary(*ch) {
            return Some(index + 1);
        }
    }
    None
}

fn is_chunk_boundary(ch: char) -> bool {
    matches!(ch, '\n' | '。' | '；' | ';' | '！' | '？' | '.' | '!' | '?')
}

fn markdown_headings(text: &str) -> Vec<(usize, String)> {
    let mut headings = Vec::new();
    let mut offset = 0usize;

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(heading) = parse_markdown_heading(trimmed) {
            headings.push((offset, heading));
        }
        offset += line.chars().count() + 1;
    }

    headings
}

fn parse_markdown_heading(line: &str) -> Option<String> {
    let marker_count = line.chars().take_while(|ch| *ch == '#').count();
    if marker_count == 0 || marker_count > 6 {
        return None;
    }

    let rest = line[marker_count..].trim();
    if rest.is_empty() {
        None
    } else {
        Some(rest.to_string())
    }
}

fn section_for_position(headings: &[(usize, String)], position: usize) -> Option<String> {
    headings
        .iter()
        .take_while(|(offset, _)| *offset <= position)
        .last()
        .map(|(_, heading)| heading.clone())
}

#[cfg(test)]
mod tests {
    use crate::models::{Document, default_access_scope};

    use super::{chunk_document, document_title_from_text};

    #[test]
    fn document_title_uses_first_markdown_heading() {
        assert_eq!(
            document_title_from_text("intro\n# 数据安全治理\n正文").as_deref(),
            Some("数据安全治理")
        );
    }

    #[test]
    fn chunk_ids_are_stable() {
        let document = Document {
            id: "doc".to_string(),
            title: "Doc".to_string(),
            source: "test".to_string(),
            access_scope: default_access_scope(),
            text: "hello world".to_string(),
        };

        let chunks = chunk_document(&document, 5, 0);

        assert_eq!(chunks[0].id, "doc:0");
        assert_eq!(chunks[1].id, "doc:1");
        assert_eq!(chunks[0].document_title, "Doc");
        assert_eq!(chunks[0].section, "Doc");
    }

    #[test]
    fn chunk_overlap_reuses_tail_context() {
        let document = Document {
            id: "doc".to_string(),
            title: "Doc".to_string(),
            source: "test".to_string(),
            access_scope: default_access_scope(),
            text: "abcdefghij".to_string(),
        };

        let chunks = chunk_document(&document, 5, 2);

        assert_eq!(chunks[0].text, "abcde");
        assert_eq!(chunks[1].text, "defgh");
        assert_eq!(chunks[2].text, "ghij");
    }

    #[test]
    fn chunking_prefers_sentence_boundaries() {
        let document = Document {
            id: "doc".to_string(),
            title: "Doc".to_string(),
            source: "test".to_string(),
            access_scope: default_access_scope(),
            text: "第一句保持完整。第二句也保持完整。第三句继续。".to_string(),
        };

        let chunks = chunk_document(&document, 14, 0);

        assert!(chunks[0].text.ends_with('。'));
        assert!(!chunks[0].text.ends_with('、'));
        assert!(
            chunks
                .iter()
                .any(|chunk| chunk.text.contains("第二句也保持完整。"))
        );
    }

    #[test]
    fn overlap_prefers_sentence_start_when_available() {
        let document = Document {
            id: "doc".to_string(),
            title: "Doc".to_string(),
            source: "test".to_string(),
            access_scope: default_access_scope(),
            text: "第一句保持完整。第二句也保持完整。第三句继续说明。第四句收尾。".to_string(),
        };

        let chunks = chunk_document(&document, 16, 8);

        assert!(chunks.len() >= 2);
        assert!(chunks[1].text.starts_with("第二句") || chunks[1].text.starts_with("第三句"));
    }

    #[test]
    fn chunks_keep_nearest_markdown_section() {
        let document = Document {
            id: "doc".to_string(),
            title: "Doc".to_string(),
            source: "test".to_string(),
            access_scope: default_access_scope(),
            text: "# 总则\n第一段内容\n## 审批流程\n第二段内容".to_string(),
        };

        let chunks = chunk_document(&document, 16, 0);

        assert_eq!(chunks[0].section, "总则");
        assert!(
            chunks
                .iter()
                .any(|chunk| chunk.section == "审批流程" && chunk.text.contains("第二段"))
        );
    }
}
