use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub addr: SocketAddr,
    pub db_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let addr = env::var("KNOWLEDGE_PILOT_ADDR")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080));

        let db_path = env::var("KNOWLEDGE_PILOT_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./data/knowledge-pilot.db"));

        Self { addr, db_path }
    }
}
