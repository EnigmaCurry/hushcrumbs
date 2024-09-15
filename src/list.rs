use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::config::load_config;

pub fn list_backups() -> io::Result<Vec<String>> {
    let config = load_config()?;

    if config.backups.is_empty() {
        return Err(io::Error::new(ErrorKind::NotFound, "No backups found"));
    }

    Ok(config.backups.keys().cloned().collect())
}

#[derive(Deserialize)]
struct Paths {
    files: HashMap<String, String>, // original_path -> backup_path
}

pub fn list_backup_files(backup_name: &str) -> io::Result<Vec<String>> {
    let config = load_config()?;
    let backup_dir = config
        .backups
        .get(backup_name)
        .ok_or(io::Error::new(ErrorKind::NotFound, "Backup not found"))?;

    let paths_file = Path::new(backup_dir).join("paths.ron");

    if !paths_file.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "No files found in the backup",
        ));
    }

    let file = File::open(paths_file)?;
    let paths: Paths = match ron::de::from_reader(file) {
        Ok(paths) => paths,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Failed to parse paths.ron",
            ))
        }
    };

    Ok(paths.files.keys().cloned().collect())
}
