use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoggerConfig {
    #[serde(default = "level::default", with = "level")]
    pub level: log::LevelFilter,
}

pub mod level {
    use serde::{Deserialize, Deserializer};
    use std::str::FromStr;

    // NOTE: based on private log::LOG_LEVEL_NAMES
    static LOG_LEVEL_NAMES: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

    pub fn default() -> log::LevelFilter {
        log::LevelFilter::Info
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<log::LevelFilter, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = log::LevelFilter::from_str(value.as_str())
            .map_err(|_| serde::de::Error::unknown_variant(value.as_str(), &LOG_LEVEL_NAMES))?;

        Ok(value)
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            level: level::default(),
        }
    }
}
