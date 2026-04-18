use crate::error::EngineError;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub groups: HashMap<char, HashSet<char>>,
}

impl Config {
    pub fn load_from_path(path: Option<PathBuf>) -> Result<Self, EngineError> {
        let file = match path {
            None => File::open("config.json"),
            Some(path) => File::open(path),
        }
        .map_err(|e| EngineError::ConfigError(format!("Could not open config file: {}", e)))?;

        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)
            .map_err(|e| EngineError::ConfigError(format!("JSON parsing error: {}", e)))?;

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), EngineError> {
        for (key, members) in &self.groups {
            if !key.is_uppercase() {
                return Err(EngineError::ConfigError(format!(
                    "Group key '{}' must be uppercase",
                    key
                )));
            }
            for member in members {
                if !member.is_lowercase() || !member.is_alphabetic() {
                    return Err(EngineError::ConfigError(format!(
                        "Group '{}' contains invalid character '{}'",
                        key, member
                    )));
                }
            }
        }
        Ok(())
    }
}
