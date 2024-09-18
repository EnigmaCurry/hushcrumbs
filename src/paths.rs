#[allow(unused_imports)]
use crate::prelude::*;

use crate::config::load_config;
use base64::engine::general_purpose::URL_SAFE;
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs::{canonicalize, File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Paths {
    pub files: HashMap<String, String>, // original_path -> backup_path
}

pub fn update_paths_ron(
    backup_name: &str,
    original_path: &Path,
    new_path: &Path,
) -> io::Result<()> {
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
        original_path.as_os_str().to_string_lossy().to_string(),
        new_path
            .file_name()
            .unwrap()
            .to_str()
            .expect("invalid to_str conversion")
            .to_string(),
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

/// Shortens the path relative to the user's home directory.
/// If the path starts with the user's home directory, replace that prefix with `~/`.
pub fn shorten_path_relative_home_directory(path: &Path) -> Option<String> {
    // Get the literal $HOME directory
    let home_dir_literal = env::var("HOME").ok().map(PathBuf::from);
    // Get the canonicalized home directory (resolves symlinks)
    let home_dir_canonical = home_dir_literal
        .as_ref()
        .and_then(|home| canonicalize(home).ok());
    // First, try to match the path against the canonicalized home directory
    if let Some(home) = home_dir_canonical {
        if path.starts_with(&home) {
            return path
                .strip_prefix(&home)
                .map(|p| format!("~/{}", p.display()))
                .ok();
        }
    }
    // If no match, try to match against the literal $HOME directory
    if let Some(home) = home_dir_literal {
        if path.starts_with(&home) {
            return path
                .strip_prefix(&home)
                .map(|p| format!("~/{}", p.display()))
                .ok();
        }
    }
    None
}

/// Shorten the path according to the following rules:
/// - If the path starts with the user's home directory, replace that prefix with `~/`.
pub fn shorten_path(path: &str) -> String {
    let absolute_path = expand_tilde_path(path).expect("failed to expand tilde");
    // Try to shorten relative to the user's home directory:
    if let Some(shortened) = shorten_path_relative_home_directory(&absolute_path) {
        return shortened;
    }
    // No match, return the original path:
    path.to_string()
}

pub fn expand_tilde_path(path: &str) -> Option<PathBuf> {
    if path.starts_with("~/") {
        if let Ok(home_dir) = env::var("HOME") {
            // Replace the `~` with the user's home directory
            let home_path = PathBuf::from(home_dir);
            return Some(home_path.join(path.trim_start_matches("~/")));
        }
    } else {
        return Some(PathBuf::from(path));
    }
    None
}

pub fn reverse_files_map(files: &HashMap<String, String>) -> HashMap<String, String> {
    let mut reversed_map = HashMap::new();
    for (key, value) in files {
        reversed_map.insert(value.clone(), key.clone());
    }
    reversed_map
}

pub fn file_hash(s: &str) -> String {
    debug!("file_hash input: {s}");
    let hash = URL_SAFE.encode(Sha256::digest(s.as_bytes())).to_string();
    let hash = hash.trim_end_matches('=').to_string();
    debug!("file_hash: {hash}");
    hash
}

/// Make relative path into absolute, even for imaginary paths.
/// This does not resolve symlinks!
/// For real paths/symlinks, use std::fs::canonicalize instead!
pub fn absolute_path(path: &str) -> PathBuf {
    let input_path = Path::new(path);
    // If it doesn't exist, join it with the current working directory
    env::current_dir().unwrap().join(input_path)
}

pub fn get_backup_paths(backup_name: &str) -> io::Result<Paths> {
    let config = load_config()?;
    let backup_dir = config
        .backups
        .get(backup_name)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Backup not found"))?;
    let paths_file = Path::new(backup_dir).join("paths.ron");
    let file = File::open(paths_file.clone())?;
    let paths: Paths = match ron::de::from_reader(file) {
        Ok(paths) => paths,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to parse paths.ron",
            ))
        }
    };
    Ok(paths)
}

pub fn set_backup_paths(backup_name: &str, paths: Paths) -> io::Result<()> {
    let config = load_config()?;
    let backup_dir = config
        .backups
        .get(backup_name)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Backup not found"))?;
    let paths_file = Path::new(backup_dir).join("paths.ron");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(paths_file)?;
    let serialized = ron::ser::to_string(&paths).expect("Failed to serialize paths");
    write!(file, "{}", serialized)?;
    Ok(())
}

pub fn check_if_file_exists_in_backup(backup_name: &str, original_path: &str) -> io::Result<bool> {
    let paths = get_backup_paths(backup_name)?;
    if paths.files.contains_key(
        absolute_path(original_path)
            .to_str()
            .expect("failed to get absolute path"),
    ) {
        Ok(true)
    } else {
        Ok(false)
    }
}
