#[allow(unused_imports)]
use crate::prelude::*;

use crate::config::load_config;
use crate::paths::{check_if_file_exists_in_backup, file_hash, update_paths_ron};
use std::fs::{canonicalize, copy, remove_file, symlink_metadata};
use std::io;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn add_to_backup(backup_name: &str, original_path: &str) -> io::Result<()> {
    let mut file_path = original_path.to_string();

    let metadata = symlink_metadata(original_path)?;
    if metadata.is_symlink() && check_if_file_exists_in_backup(backup_name, &file_path)? {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "File already exists in backup.",
        ));
    }

    // Canonicalize the path
    let absolute_path = canonicalize(&file_path).expect("could not get absolute file_path");
    file_path = absolute_path
        .to_str()
        .expect("could not get absolute path")
        .to_string();

    let config = load_config()?;
    //debug!("loaded config");
    let backup_dir = config.backups.get(backup_name).ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Backup does not exist",
    ))?;
    //debug!("backup found");

    // Hash the original path to form the backup file id:
    let new_path = Path::new(backup_dir).join(file_hash(
        absolute_path.to_str().expect("failed to_str on path"),
    ));
    debug!("new_path: {new_path:?}");
    debug!("file_path: {file_path:?}");
    let metadata = symlink_metadata(original_path)?;
    //debug!("metadata loaded");
    if metadata.is_symlink() {
        return Err(io::Error::new(io::ErrorKind::Other, "Cannot add symlink"));
    }

    copy(original_path, &new_path)?;
    remove_file(original_path)?;
    //debug!("moved");
    symlink(&new_path, absolute_path.clone())?;
    //debug!("symlinked");

    // Update paths.ron with the original path
    update_paths_ron(backup_name, Path::new(&file_path.clone()), &new_path)
}
