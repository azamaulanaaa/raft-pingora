mod config;
mod raft;
mod timer;

use crate::{config::Config, raft::Raft, timer::Timer};
use anyhow::Result;
use simple_logger::SimpleLogger;

static CONFIG_PATH: &str = "./config.toml";

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_file(CONFIG_PATH)?;

    SimpleLogger::new().init()?;
    log::set_max_level(config.logger.level);

    let raft = Raft::new(&config.raft)?;

    let mut timer = Timer::new(std::time::Duration::from_secs(1), async || {
        log::info!("Hello world!");
    });
    timer.start()?;
    timer.wait();
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    timer.stop()?;

    Ok(())
}
