use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;

use crate::config::load_config;

#[derive(Serialize, Deserialize, Default)]
pub struct Paths {
    files: HashMap<String, String>, // original_path -> backup_path
}

pub fn update_paths_ron(backup_name: &str, original_path: &str, new_path: &Path) -> io::Result<()> {
    let config = load_config()?;
    let backup_dir = config.backups.get(backup_name).ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Backup does not exist",
    ))?;

    let paths_file = Path::new(backup_dir).join("paths.ron");

    // Load or initialize the paths file
    let mut paths: Paths = if paths_file.exists() {
        let file = File::open(&paths_file)?;
        match ron::de::from_reader(file) {
            Ok(paths) => paths,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Failed to parse paths file",
                ))
            }
        }
    } else {
        Paths::default()
    };

    // Add the new entry to the paths file
    paths.files.insert(
        original_path.to_string(),
        new_path.to_str().unwrap().to_string(),
    );

    // Write the updated paths back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(paths_file)?;
    let serialized = ron::ser::to_string(&paths).expect("Failed to serialize paths");
    write!(file, "{}", serialized)?;

    Ok(())
}
