use crate::paths::{
    absolute_path, file_hash, get_backup_paths, reverse_files_map, set_backup_paths, Paths,
};
#[allow(unused_imports)]
use crate::prelude::*;

use crate::config::load_config;
use inquire::Confirm;
use std::fs::{self, canonicalize, File, OpenOptions};
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
        let backup_path = Path::new(backup_dir).join(Path::new(&backup_path));
        //debug!("backup_path: {backup_path:?}");
        let backup = canonicalize(backup_path).expect("could not get absolute path");

        // Check if the original path is a symlink and if it already points to the correct backup:
        if original.exists() {
            if original.is_symlink() {
                match fs::read_link(original) {
                    Ok(link_target) => {
                        // If the symlink is already pointing to the correct target, skip it
                        if link_target == backup {
                            info!("Valid symlink already exists: {}", original.display());
                            continue;
                        }
                    }
                    Err(err) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Failed to read symlink {}: {}", original.display(), err),
                        ));
                    }
                }
            } else if !overwrite {
                // Check if the original path exists and handle overwrite logic
                let confirm = Confirm::new(&format!(
                    "File {} already exists. Overwrite?",
                    original.display()
                ))
                .with_default(false)
                .prompt();

                match confirm {
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

fn remove_backup_entry(backup_name: &str, original_path: &str) -> io::Result<()> {
    let mut paths = get_backup_paths(backup_name).expect("Could not get backup paths");
    // Remove the file from the paths map
    debug!("paths: {paths:?}");
    debug!("original_path: {original_path:?}");
    paths.files.remove(&original_path.to_string());
    set_backup_paths(backup_name, paths)
}

fn destroy_backup_file(backup_name: &str, original_path: &str) -> io::Result<()> {
    debug!("original_path: {original_path:?}");
    let paths = get_backup_paths(backup_name)?;
    debug!("paths: {paths:?}");
    let id = paths
        .files
        .get(original_path)
        .expect("failed to get backup file entry");
    let config = load_config()?;
    let backup_dir = config
        .backups
        .get(backup_name)
        .ok_or(io::Error::new(ErrorKind::NotFound, "Backup not found"))?;
    let backup_file = Path::new(backup_dir).join(Path::new(id));
    debug!("backup_file: {backup_file:?}");
    remove_backup_entry(backup_name, original_path)?;
    Ok(())
}

pub fn remove_from_backup(backup_name: &str, original_path: &str, delete: bool) -> io::Result<()> {
    debug!("loaded config");
    let config = load_config()?;
    let backup_dir = config
        .backups
        .get(backup_name)
        .ok_or(io::Error::new(ErrorKind::NotFound, "Backup not found"))?;
    debug!("got backup dir");

    let paths_file = Path::new(backup_dir).join("paths.ron");

    if !paths_file.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "No backup records found in paths.ron",
        ));
    }

    let paths = get_backup_paths(backup_name).expect("failed to get backup paths");

    let canonical_path;
    match canonicalize(original_path) {
        Err(_) => {
            if !delete {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("The existing path does not exist. To remove this entry from the backup, without restoring it, add the --delete argument.")));
            } else {
                destroy_backup_file(
                    backup_name,
                    absolute_path(original_path)
                        .to_str()
                        .expect("failed to_str"),
                )
                .expect("failed to remove backup file");
                return Ok(());
            }
        }
        Ok(p) => canonical_path = p,
    }
    let mut original = Path::new(original_path);
    let abs_path;

    let files = reverse_files_map(&paths.files);

    if canonical_path.as_path().parent().expect("failed dirname()") == Path::new(backup_dir) {
        let backup = canonical_path.clone();
        debug!("backup: {backup:?}");
        let id = backup
            .file_name()
            .expect("failed file_name()")
            .to_str()
            .expect("failed to_str()");
        debug!("id: {id}");
        match files.get(id) {
            None => {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    format!("File not found in backup: {original:?}"),
                ))
            }
            Some(f) => {
                original = Path::new(f);
                let a = absolute_path(original_path);
                abs_path = a.to_str().expect("could not get absolute path");

                debug!("before remove");
                if delete {
                    // Remove the symlink at the original path
                    if original.exists() && original.is_symlink() {
                        fs::remove_file(original)?;
                    }
                    debug!("removed symlink");
                    return Ok(());
                } else {
                    // Restore the original file by copying it from the backup
                    if original.exists() && original.is_symlink() {
                        fs::remove_file(original)?; // Remove the symlink
                        debug!("removed_symlink");
                    }
                    debug!("backup: {backup:?}");
                    debug!("original: {original:?}");
                    fs::copy(backup, original)?; // Restore the original file
                    debug!("copied");
                }

                destroy_backup_file(backup_name, abs_path).expect("failed to remove backup file");
                info!("File restored and removed from backup: {original_path:?}");
                Ok(())
            }
        }
    } else {
        if original.exists() && delete {
            destroy_backup_file(
                backup_name,
                absolute_path(original_path)
                    .to_str()
                    .expect("failed to_str"),
            )
        } else if original.exists() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("A conflicting non-backup file exists in the original path: '{}'. To remove this entry from the backup without restoring it, add the --delete argument.", original_path)));
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("The existing path does not exist. To remove this entry from the backup, without restoring it, add the --delete argument.")));
        }
    }
}
