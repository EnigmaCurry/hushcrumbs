use crate::paths::{expand_tilde_path, shorten_path};
#[allow(unused_imports)]
use crate::prelude::*;
use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::json;
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

pub fn list_backups(output_as_json: bool) -> io::Result<()> {
    match get_backups() {
        Ok(backups) => {
            if output_as_json {
                // Collect the backups into a vector of BackupData
                // let json_backups: Vec<BackupData> = backups
                //     .into_iter()
                //     .map(|backup| BackupData {
                //         name: backup.clone(),
                //         path: get_backup_path(&backup).unwrap_or_else(|e| e.to_string()),
                //     })
                //     .collect();
                let json_backups: Vec<serde_json::Value> = backups
                    .into_iter()
                    .map(|backup| {
                        json! ({
                            "name": backup.clone(),
                            "path": get_backup_path(&backup).unwrap_or_else(|e| e.to_string()),
                        })
                    })
                    .collect();

                let json_output = json!({"backups": json_backups});

                println!("{}", json_output);
            } else {
                let mut table = get_table(vec!["Backup Name", "Backup Path"]);
                for backup in backups {
                    table.add_row(row![
                        backup,
                        get_backup_path(&backup).unwrap_or_else(|e| e.to_string())
                    ]);
                }
                table.printstd();
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct Paths {
    files: IndexMap<String, String>, // original_path -> backup_path
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

    if paths.files.len() == 0 {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "No files found in the backup",
        ));
    }

    Ok(paths.files.keys().cloned().collect())
}

pub fn list_backup_files(backup_name: &str, output_as_json: bool) -> io::Result<()> {
    match get_backup_files(backup_name) {
        Ok(files) => {
            if output_as_json {
                let expanded_files: Vec<String> = files
                    .iter()
                    .map(|file| {
                        let exp = expand_tilde_path(file).expect("failed to expand path");
                        exp.to_str().expect("failed to stringify path").to_string()
                    })
                    .collect();
                debug!("expanded_files: {:?}", expanded_files);
                let json_output = json!({
                    "backup_name": backup_name,
                    "files": expanded_files,
                });

                println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
            } else {
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
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
