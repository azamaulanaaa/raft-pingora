use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub logger: Logger,
}

impl Config {
    pub fn read_file<'a>(path: &'a str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str::<Self>(&content)?;
        return Ok(config);
    }
}

#[derive(Deserialize)]
pub struct Logger {
    #[serde(default = "logger::level::default", with = "logger::level")]
    pub level: log::LevelFilter,
}

mod logger {
    pub mod level {
        use serde::{Deserialize, Deserializer};
        use std::str::FromStr;

        // NOTE: based on private log::LOG_LEVEL_NAMES
        static LOG_LEVEL_NAMES: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

        pub fn default() -> log::LevelFilter {
            return log::LevelFilter::Info;
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<log::LevelFilter, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = String::deserialize(deserializer)?;
            let value = log::LevelFilter::from_str(value.as_str())
                .map_err(|_| serde::de::Error::unknown_variant(value.as_str(), &LOG_LEVEL_NAMES))?;

            return Ok(value);
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        return Logger {
            level: logger::level::default(),
        };
    }
}
