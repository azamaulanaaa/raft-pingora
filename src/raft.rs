use crate::config::RaftConfig;
use anyhow::{Error, Result, anyhow};
use raft::{Config, raw_node::RawNode, storage::MemStorage};

pub struct Raft {
    raw_node: RawNode<MemStorage>,
}

impl Raft {
    pub fn new(config: &RaftConfig) -> Result<Self> {
        let config = Config::try_from(config)?;
        let storage = MemStorage::new();
        let raw_node = RawNode::with_default_logger(&config, storage)?;

        Ok(Raft { raw_node })
    }
}

impl TryFrom<&RaftConfig> for Config {
    type Error = Error;

    fn try_from(value: &RaftConfig) -> std::result::Result<Self, Self::Error> {
        if value.id == 0 {
            return Err(anyhow!("id must not be positif integer"));
        }

        Ok(Config {
            id: value.id,
            ..Default::default()
        })
    }
}
