mod config;

use anyhow::Result;
use config::Config;
use simple_logger::SimpleLogger;

const CONFIG_PATH: &str = "./config.toml";

fn main() -> Result<()> {
    SimpleLogger::new().init()?;

    let config = Config::read_file(CONFIG_PATH)?;
    log::set_max_level(config.logger.level);

    log::info!("testing");

    return Ok(());
}
