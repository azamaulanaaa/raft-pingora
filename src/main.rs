mod config;
mod raft;

use crate::{config::Config, raft::Raft};
use anyhow::Result;
use simple_logger::SimpleLogger;

static CONFIG_PATH: &str = "./config.toml";

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_file(CONFIG_PATH)?;

    SimpleLogger::new().init()?;
    log::set_max_level(config.logger.level);

    let raft = Raft::new(&config.raft)?;

    Ok(())
}
