use crate::config::load_config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::BufRead;
use std::io::{self, ErrorKind, Write};
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct Paths {
    files: HashMap<String, String>, // original_path -> backup_path
}

pub fn restore_backup(backup_name: &str, copy: bool, overwrite: bool) -> io::Result<()> {
    let config = load_config()?;
    let backup_dir = config
        .backups
        .get(backup_name)
        .ok_or(io::Error::new(ErrorKind::NotFound, "Backup not found"))?;

    let paths_file = Path::new(backup_dir).join("paths.ron");

    if !paths_file.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "No backup records found in paths.ron",
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

    for (original_path, backup_path) in paths.files {
        let original = Path::new(&original_path);
        let backup = Path::new(&backup_path);

        // Check if the original path exists and handle overwrite logic
        if original.exists() && !overwrite {
            print!(
                "File {} already exists. Overwrite? (y/N): ",
                original.display()
            );
            let mut input = String::new();
            io::stdin().lock().read_line(&mut input)?;
            if input.trim().to_lowercase() != "y" {
                continue;
            }
        }

        // Copy or create a symlink based on the --copy flag
        if copy {
            fs::copy(backup, original)?;
        } else {
            if original.exists() {
                fs::remove_file(original)?; // Remove the existing file if it exists
            }
            std::os::unix::fs::symlink(backup, original)?;
        }
    }

    Ok(())
}

pub fn remove_from_backup(
    backup_name: &str,
    original_path: Option<&str>,
    delete: bool,
) -> io::Result<()> {
    let config = load_config()?;
    let backup_dir = config
        .backups
        .get(backup_name)
        .ok_or(io::Error::new(ErrorKind::NotFound, "Backup not found"))?;

    let paths_file = Path::new(backup_dir).join("paths.ron");

    if !paths_file.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "No backup records found in paths.ron",
        ));
    }

    let file = File::open(paths_file.clone())?;
    let mut paths: Paths = match ron::de::from_reader(file) {
        Ok(paths) => paths,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Failed to parse paths.ron",
            ))
        }
    };

    if let Some(original_path) = original_path {
        let backup_path = paths.files.get(original_path).ok_or(io::Error::new(
            ErrorKind::NotFound,
            "File not found in the backup",
        ))?;

        let original = Path::new(original_path);
        let backup = Path::new(backup_path);

        if delete {
            // Remove the symlink at the original path
            if original.exists() && original.is_symlink() {
                fs::remove_file(original)?;
            }
        } else {
            // Restore the original file by copying it from the backup
            if original.exists() && original.is_symlink() {
                fs::remove_file(original)?; // Remove the symlink
            }
            fs::copy(backup, original)?; // Restore the original file
        }

        // Remove the file from the paths map
        paths.files.remove(original_path);

        // Update the paths.ron file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(paths_file)?;
        let serialized = ron::ser::to_string(&paths).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize paths: {}", e),
            )
        })?;
        file.write_all(serialized.as_bytes())?;
    }

    Ok(())
}
