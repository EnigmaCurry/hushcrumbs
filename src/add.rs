use rand::distributions::Alphanumeric;

use crate::config::load_config;
use crate::paths::update_paths_ron;
use rand::Rng;
use std::fs::{rename, symlink_metadata};
use std::io;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn add_to_backup(backup_name: &str, file_path: &str) -> io::Result<()> {
    let config = load_config()?;
    let backup_dir = config.backups.get(backup_name).ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Backup does not exist",
    ))?;

    // Generate a random 10-character alphanumeric directory name
    let random_dir: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let new_path = Path::new(backup_dir).join(&random_dir);

    let metadata = symlink_metadata(file_path)?;
    if metadata.is_symlink() {
        return Err(io::Error::new(io::ErrorKind::Other, "Cannot add symlink"));
    }

    rename(file_path, &new_path)?;
    symlink(&new_path, file_path)?;

    // Update paths.ron with the original path
    update_paths_ron(backup_name, file_path, &new_path)
}
