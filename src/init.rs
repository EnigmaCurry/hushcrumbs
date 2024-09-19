use crate::config::load_config;
use crate::paths::get_backup_paths;
#[allow(unused_imports)]
use crate::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self};
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    backups: HashMap<String, String>, // Backup name -> path
}

pub fn init_backup(backup_name: &str, path: Option<&str>) -> io::Result<()> {
    debug!("init backup: {:?}", &backup_name);
    // Resolve the provided path or default to the current directory
    let backup_path = Path::new(path.unwrap_or("."));

    // Check if the path already exists
    if backup_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Backup path already exists: {backup_path:?}"),
        ));
    }
    debug!("creating directory: {:?}", &backup_path);
    // Create the backup directory
    fs::create_dir(backup_path)?;

    // Convert to absolute path
    let absolute_backup_path = fs::canonicalize(backup_path)?;

    // Update the config with the absolute path
    update_config(backup_name, &absolute_backup_path)?;

    Ok(())
}

pub fn update_config(backup_name: &str, path: &Path) -> io::Result<()> {
    let config_dir = dirs::config_dir().unwrap().join("secrets");
    let config_path = config_dir.join("config.ron");

    // Ensure the parent directory exists
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    // Load or initialize the configuration
    let mut config = load_config()?;

    // Add or update the backup name and path in the config
    config
        .backups
        .insert(backup_name.to_string(), path.to_str().unwrap().to_string());

    // Write the updated config to the file
    let mut file = fs::File::create(config_path)?;
    write!(file, "{}", ron::ser::to_string(&config).unwrap())?;
    Ok(())
}

pub fn deinit_backup(backup_name: &str) -> io::Result<()> {
    // Get the path to the config file
    let config_path = dirs::config_dir().unwrap().join("secrets/config.ron");

    // Load the existing config
    let mut config = load_config()?;
    let mut remove = || {
        // Remove the backup from the config
        if config.backups.remove(backup_name).is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Backup not found: {backup_name}"),
            ));
        }
        // Save the updated config back to the file
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&config_path)?;
        let serialized = ron::ser::to_string(&config).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize config: {}", e),
            )
        })?;
        file.write_all(serialized.as_bytes())?;
        println!(
            "Backup '{}' has been removed from the configuration.",
            backup_name
        );
        Ok(())
    };
    match get_backup_paths(backup_name) {
        Ok(paths) => {
            if paths.files.len() > 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Backup '{backup_name}' still has active symlinks. deinit is prevented in this state. "),
                ));
            } else {
                remove()
            }
        }
        Err(_) => remove(),
    }
}
