use add::add_to_backup;
use clap::{Arg, Command};
use env_logger::Env;
use init::{deinit_backup, init_backup};
use list::{list_backup_files, list_backups};
use log::Level;
use restore::{remove_from_backup, restore_backup};
mod add;
mod config;
mod init;
mod list;
mod paths;
mod prelude;
mod restore;

#[macro_use]
extern crate prettytable;

use prelude::*;

fn main() {
    let mut cmd = Command::new("secrets")
        .version("1.0")
        .author("Author Name <email@example.com>")
        .about("A CLI backup tool")
        .arg(
            Arg::new("log")
                .long("log")
                .num_args(1)
                .value_name("LEVEL")
                .value_parser(["trace", "debug", "info", "warn", "error"])
                .help("Sets the log level, overriding the RUST_LOG environment variable"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .help("Sets the log level to debug")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("init")
                .about("Creates a new backup directory")
                .arg(Arg::new("BACKUP_NAME").required(true))
                .arg(Arg::new("PATH").required(true)),
        )
        .subcommand(
            Command::new("deinit")
                .about("Restores all original files and unconfigures the backup directory")
                .arg(Arg::new("BACKUP_NAME").required(true)),
        )
        .subcommand(
            Command::new("add")
                .about("Adds a file to the backup and creates a symlink")
                .arg(Arg::new("BACKUP_NAME").required(true))
                .arg(Arg::new("PATH").required(true)),
        )
        .subcommand(
            Command::new("restore")
                .about("Restores backup files")
                .arg(Arg::new("BACKUP_NAME").required(true))
                .arg(Arg::new("copy").long("copy"))
                .arg(Arg::new("overwrite").long("overwrite")),
        )
        .subcommand(
            Command::new("rm")
                .visible_alias("remove")
                .about("Removes a file from the backup")
                .arg(Arg::new("BACKUP_NAME").required(true))
                .arg(Arg::new("PATH").required(false))
                .arg(
                    Arg::new("delete")
                        .long("delete")
                        .action(clap::ArgAction::SetTrue)
                        .help(
                            "If set, the file will be deleted from the backup without restoring it",
                        ),
                ),
        )
        .subcommand(
            Command::new("ls")
                .visible_alias("list")
                .about("Lists backups or files in a backup")
                .arg(Arg::new("BACKUP_NAME").required(false)),
        )
        .subcommand(
            Command::new("commit")
                .about("Commits a backup (placeholder)")
                .arg(Arg::new("BACKUP_NAME").required(true)),
        )
        .subcommand(
            Command::new("push")
                .about("Pushes a backup (placeholder)")
                .arg(Arg::new("BACKUP_NAME").required(true)),
        );
    let matches = cmd.clone().get_matches();

    // Configure logging:
    env_logger::Builder::from_env(
        Env::default().default_filter_or(
            matches
                .get_one::<String>("log")
                .map(|s| s.as_str())
                .unwrap_or("info"),
        ),
    )
    .format_timestamp(None)
    .init();

    // Print help if no subcommand is given:
    if matches.subcommand_name().is_none() {
        cmd.print_help().unwrap();
        println!();
        return;
    }

    // Handle the subcommands:
    let exit_code = match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            let path = sub_matches.get_one::<String>("PATH");
            match init_backup(backup_name, path.map(|s| s.as_str())) {
                Ok(_) => {
                    info!("Backup '{}' initialized successfully.", backup_name);
                    0
                }
                Err(e) => {
                    eprintln!("Error initializing backup: {}", e);
                    1
                }
            }
        }
        Some(("deinit", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            match deinit_backup(backup_name) {
                Ok(_) => {
                    info!("Backup '{}' removed from config.", backup_name);
                    0
                }
                Err(e) => {
                    eprintln!("Error uninitializing backup: {}", e);
                    1
                }
            }
        }
        Some(("add", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            let file_path = sub_matches.get_one::<String>("PATH").unwrap();
            match add_to_backup(backup_name, file_path) {
                Ok(_) => {
                    info!("File '{}' added to backup '{}'.", file_path, backup_name);
                    0
                }
                Err(e) => {
                    eprintln!("Error adding file to backup: {}", e);
                    1
                }
            }
        }
        Some(("restore", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            let copy = sub_matches.contains_id("copy");
            let overwrite = sub_matches.contains_id("overwrite");
            match restore_backup(backup_name, copy, overwrite) {
                Ok(_) => {
                    info!("Backup '{}' restored successfully.", backup_name);
                    0
                }
                Err(e) => {
                    eprintln!("Error restoring backup: {}", e);
                    1
                }
            }
        }
        Some(("rm", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            let file_path = sub_matches.get_one::<String>("PATH");
            let delete = sub_matches.get_flag("delete");
            match file_path {
                Some(f) => match remove_from_backup(backup_name, f.as_str(), delete) {
                    Ok(_) => 0,
                    Err(e) => {
                        eprintln!("Error removing file from backup: {}", e);
                        1
                    }
                },
                None => {
                    eprintln!("File does not exist: {file_path:?}");
                    1
                }
            }
        }
        Some(("ls", sub_matches)) => {
            if let Some(backup_name) = sub_matches.get_one::<String>("BACKUP_NAME") {
                list_backup_files(backup_name);
            } else {
                list_backups();
            }
            0
        }
        Some(("commit", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            info!("Commit backup '{}' (not implemented).", backup_name);
            0
        }
        Some(("push", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            info!("Push backup '{}' (not implemented).", backup_name);
            0
        }
        _ => 1,
    };

    std::process::exit(exit_code);
}
