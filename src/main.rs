mod config;

use anyhow::Result;
use config::Config;
use simple_logger::SimpleLogger;

static CONFIG_PATH: &str = "./config.toml";

fn main() -> Result<()> {
    let config = Config::read_file(CONFIG_PATH)?;

    SimpleLogger::new().init()?;
    log::set_max_level(config.logger.level);

    log::info!("testing");

    return Ok(());
}
