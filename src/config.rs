#[allow(unused_imports)]
use crate::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub backups: HashMap<String, String>, // Backup name -> path
}

pub fn load_config() -> io::Result<Config> {
    let config_path = get_config_path();

    if !config_path.exists() {
        return Ok(Config::default()); // Return an empty config if the file doesn't exist
    }

    let file = File::open(&config_path)?;
    let config: Config = match ron::de::from_reader(file) {
        Ok(config) => config,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Failed to parse config",
            ))
        }
    };

    Ok(config)
}

fn get_config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("Failed to find config directory");
    config_dir.push("secrets/config.ron");
    config_dir
}
