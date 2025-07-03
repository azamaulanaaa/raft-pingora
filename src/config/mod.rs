mod logger;
mod raft;

pub use logger::LoggerConfig;
pub use raft::RaftConfig;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub logger: LoggerConfig,
    #[serde(default)]
    pub raft: RaftConfig,
}

impl Config {
    pub fn read_file(path: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str::<Self>(&content)?;
        Ok(config)
    }
}
