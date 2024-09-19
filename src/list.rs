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
        eprintln!();
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
            let mut table = get_table(vec!["Available Backups:"]);
            for backup in backups {
                table.add_row(row![backup]);
            }
            println!();
            table.printstd();
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e)
        }
    }
}

#[derive(Deserialize)]
struct Paths {
    files: HashMap<String, String>, // original_path -> backup_path
}

fn get_backup_files(backup_name: &str) -> io::Result<Vec<String>> {
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

pub fn list_backup_files(backup_name: &str) -> io::Result<()> {
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
                eprintln!();
                eprintln!("No files found in backup.");
                Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "No files found in backup",
                ))
            }
        }
        Err(e) => {
            eprintln!();
            eprintln!("{}: {backup_name}", e);
            Err(e)
        }
    }
}
