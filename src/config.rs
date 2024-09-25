#[allow(unused_imports)]
use crate::prelude::*;

use crate::get_options;
use indexmap::IndexMap;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub backups: IndexMap<String, String>, // Backup name -> path
}

pub fn load_config() -> io::Result<Config> {
    // Config path is specified by command line argument:
    let config_path: PathBuf = get_options().config_file.clone();
    // If no config exists, return the default config:
    if !config_path.exists() {
        debug!("No config file found. Loading the default/blank config.");
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
    debug!("Config loaded: {config_path:?}");
    Ok(config)
}

pub fn save_config(config: &Config) -> io::Result<()> {
    // Config path is specified by command line argument:
    let config_path: PathBuf = get_options().config_file.clone();
    let serialized = ron::ser::to_string(config).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to serialize config: {}", e),
        )
    })?;
    let mut file = File::create(config_path.clone())?;
    file.write_all(serialized.as_bytes())?;
    debug!("Config saved: {config_path:?}");
    Ok(())
}
