use crate::config::{load_config, save_config};
use crate::get_options;
use crate::paths::get_backup_paths;
#[allow(unused_imports)]
use crate::prelude::*;

use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

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
    let config_path: PathBuf = get_options().config_file.clone();
    let config_dir = config_path
        .parent()
        .expect("Could not discover config file parent directory");

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

    // Save the updated config using the common save_config method
    save_config(&config)
}

pub fn deinit_backup(backup_name: &str) -> io::Result<()> {
    // Load the existing config
    let mut config = load_config()?;
    let mut remove = || {
        // Remove the backup from the config
        if config.backups.shift_remove(backup_name).is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Backup not found: {backup_name}"),
            ));
        }

        // Save the updated config back to the file using the common save_config method
        save_config(&config)?;

        println!(
            "Backup '{}' has been removed from the configuration.",
            backup_name
        );
        Ok(())
    };

    match get_backup_paths(backup_name) {
        Ok(paths) => {
            if !paths.files.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Backup '{backup_name}' still has active symlinks. deinit is prevented in this state. "),
                ))
            } else {
                remove()
            }
        }
        Err(_) => remove(),
    }
}
