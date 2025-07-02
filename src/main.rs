mod config;

use anyhow::Result;
use config::Config;

fn main() -> Result<()> {
    println!("Hello, world!");
    let config = Config::read_file("./config.toml")?;

    return Ok(());
}
