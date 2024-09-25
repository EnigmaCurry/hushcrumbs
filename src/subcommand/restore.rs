use crate::paths::Paths;
#[allow(unused_imports)]
use crate::prelude::*;

use crate::config::load_config;
use crate::confirm::{confirm, ConfirmProps};
use std::fs::{self, canonicalize, File};
use std::io::{self, ErrorKind};
use std::path::Path;

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
            format!("{backup_dir}/paths.ron is missing"),
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
        let backup_path = Path::new(backup_dir).join(Path::new(&backup_path));
        //debug!("backup_path: {backup_path:?}");
        let backup = canonicalize(backup_path).expect("could not get absolute path");

        // Check if the original path is a symlink and if it already points to the correct backup:
        if original.exists() {
            if original.is_symlink() {
                let link_target = fs::read_link(original).unwrap();
                if link_target == backup {
                    info!("Valid symlink already exists: {}", original.display());
                    continue;
                }
            } else if !overwrite {
                // Check if the original path exists and handle overwrite logic
                match confirm(ConfirmProps {
                    message: format!("File {} already exists. Overwrite?", original.display()),
                    ..Default::default()
                }) {
                    Ok(true) => (),        // User chose to overwrite
                    Ok(false) => continue, // User chose not to overwrite
                    Err(_) => {
                        return Err(io::Error::new(
                            ErrorKind::Interrupted,
                            "Prompt was cancelled or failed",
                        ))
                    }
                }
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
