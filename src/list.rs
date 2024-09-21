use crate::paths::{expand_tilde_path, shorten_path};
#[allow(unused_imports)]
use crate::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::config::load_config;
use prettytable::{format::consts as fmt, Cell, Row, Table};

fn get_backups() -> io::Result<Vec<String>> {
    let config = load_config()?;
    if config.backups.is_empty() {
        return Err(io::Error::new(ErrorKind::NotFound, "No backups found."));
    }
    Ok(config.backups.keys().cloned().collect())
}

fn get_table(titles: Vec<&str>) -> Table {
    let mut table = Table::new();
    table.set_format(*fmt::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(titles.iter().map(|x| Cell::new(x)).collect()));
    table
}

pub fn list_backups() -> io::Result<()> {
    match get_backups() {
        Ok(backups) => {
            let mut table = get_table(vec!["Backup Name", "Backup Path"]);
            for backup in backups {
                table.add_row(row![
                    backup,
                    get_backup_path(&backup).unwrap_or_else(|e| e.to_string())
                ]);
            }
            table.printstd();
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct Paths {
    files: HashMap<String, String>, // original_path -> backup_path
}

fn get_backup_path(backup_name: &str) -> io::Result<String> {
    let config = load_config()?;
    match config
        .backups
        .get(backup_name)
        .ok_or(io::Error::new(ErrorKind::NotFound, "Backup not found"))
        .cloned()
    {
        Ok(path) => {
            if Path::new(&path).exists() {
                Ok(path)
            } else {
                Err(io::Error::new(
                    ErrorKind::InvalidData,
                    format!("Backup directory is missing: {path}"),
                ))
            }
        }
        Err(e) => Err(e),
    }
}

fn get_backup_files(backup_name: &str) -> io::Result<Vec<String>> {
    let backup_dir = get_backup_path(backup_name)?;
    let paths_file = Path::new(&backup_dir).join("paths.ron");

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

pub fn list_backup_files(backup_name: &str) -> io::Result<()> {
    let backup_dir = &get_backup_path(backup_name)?;
    let backup_dir = Path::new(backup_dir);
    if !backup_dir.exists() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("Backup directory does not exist: {backup_dir:?}"),
        ));
    }
    match get_backup_files(backup_name) {
        Ok(files) => {
            if !files.is_empty() {
                let mut table = get_table(vec![&format!(
                    "Local files contained in backup ({backup_name}):"
                )]);
                for file in files {
                    let exp = expand_tilde_path(&file).expect("failed to expand path");
                    let f = exp.as_path().to_str().expect("failed to stringify path");
                    table.add_row(row![shorten_path(f)]);
                }
                println!();
                table.printstd();
                Ok(())
            } else {
                Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "No files found in backup",
                ))
            }
        }
        Err(e) => Err(e),
    }
}
