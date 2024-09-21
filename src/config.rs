#[allow(unused_imports)]
use crate::prelude::*;

use crate::get_options;
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
    // Config path is specified by command line argument:
    let config_path: PathBuf = get_options().config_file.clone();
    // If no config exists, return the default config:
    if !config_path.exists() {
        return Ok(Config::default());
    }
    // Load the config file:
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
