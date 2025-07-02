use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {}

impl Config {
    pub fn read_file<'a>(path: &'a str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str::<Self>(&content)?;
        return Ok(config);
    }
}
