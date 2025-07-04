mod clock;
mod config;
mod raft;

use crate::{clock::Clock, config::Config, raft::Raft};
use anyhow::Result;
use simple_logger::SimpleLogger;

static CONFIG_PATH: &str = "./config.toml";

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_file(CONFIG_PATH)?;

    SimpleLogger::new().init()?;
    log::set_max_level(config.logger.level);

    let raft = Raft::new(&config.raft)?;

    let mut clock = Clock::new(std::time::Duration::from_secs(1), async || {
        log::info!("Hello world!");
    });
    clock.start()?;
    clock.wait();
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    clock.stop()?;

    Ok(())
}
