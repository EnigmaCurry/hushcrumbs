use clap::{Arg, Command};
use confirm::{confirm, ConfirmProps};
use once_cell::sync::OnceCell;
use std::{path::PathBuf, str::FromStr};
use subcommand::{
    add::add_to_backup,
    init::{deinit_backup, init_backup},
    list::{list_backup_files, list_backups},
    remove::remove_from_backup,
    restore::restore_backup,
};
mod config;
mod confirm;
mod paths;
mod prelude;
mod subcommand;

#[macro_use]
extern crate prettytable;

use prelude::*;

/// Options is a subset of the command line options that need to be shared globally:
/// (cloning the entire ArgMatches object did not behave well, so this is its proxy:)
#[derive(Default, Debug)]
pub struct Options {
    config_file: PathBuf,
    no_confirm: bool,
}
/// Globally shared Options instance:
static OPTIONS: OnceCell<Options> = OnceCell::new();
pub fn get_options() -> &'static Options {
    OPTIONS.get().expect("Options has not been set.")
}

fn main() {
    let mut cmd = Command::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .global(true)
                .num_args(1)
                .value_name("CONFIG_FILE")
                .help("Sets the path to the global config file.")
                .default_value(config::get_default_config_path()),
        )
        .arg(
            Arg::new("log")
                .long("log")
                .global(true)
                .num_args(1)
                .value_name("LEVEL")
                .value_parser(["trace", "debug", "info", "warn", "error"])
                .help("Sets the log level, overriding the RUST_LOG environment variable."),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .global(true)
                .help("Sets the log level to debug.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-confirm")
                .long("no-confirm")
                .global(true)
                .help("Disables all interactive confirmation (careful!)")
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
                .arg(Arg::new("copy").long("copy").help("Restore file by copying, rather than symlinking").action(clap::ArgAction::SetTrue))
                .arg(Arg::new("overwrite").long("overwrite")),
        )
        .subcommand(
            Command::new("rm")
                .visible_alias("remove")
                .about("Removes a file from the backup")
                .arg(Arg::new("BACKUP_NAME").required(true))
                .arg(Arg::new("PATH").required(true))
                .arg(
                    Arg::new("delete")
                        .long("delete")
                        .action(clap::ArgAction::SetTrue)
                        .help(
                            "Delete the file AND the backup, without restoring it (requires confirmation unless also --no-confirm)",
                        ),
                ),
        )
        .subcommand(
            Command::new("ls")
                .visible_alias("list")
                .about("Lists backups or files in a backup")
                .arg(Arg::new("BACKUP_NAME").required(false))
                .arg(
                    Arg::new("json")
                        .long("json")
                        .action(clap::ArgAction::SetTrue)
                        .help("Output JSON instead of pretty tables."),
                ),
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

    // Set global options for sharing a subset of the args with other modules:
    OPTIONS
        .set(Options {
            config_file: PathBuf::from(matches.get_one::<String>("config").expect("no config arg")),
            no_confirm: matches.get_flag("no-confirm"),
        })
        .expect("Options can only be set once");

    // Configure logging:
    let log_level = if matches.get_flag("verbose") {
        Some("debug".to_string())
    } else {
        matches.get_one::<String>("log").cloned()
    };
    // Use RUST_LOG env var if no command-line option is provided
    let log_level = log_level.or_else(|| std::env::var("RUST_LOG").ok());
    // Fallback to "info" if neither command-line option nor env var is set
    let log_level = log_level.unwrap_or_else(|| "info".to_string());
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::from_str(&log_level).unwrap_or(log::LevelFilter::Info))
        .format_timestamp(None)
        .init();
    debug!("logging initialized.");

    // Print help if no subcommand is given:
    if matches.subcommand_name().is_none() {
        cmd.print_help().unwrap();
        println!();
        return;
    }

    // Handle the subcommands:
    eprintln!("");
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
            let copy = sub_matches.get_flag("copy");
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
            let remove = || match file_path {
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
            };
            if delete {
                match confirm(ConfirmProps {
                    message: "Do you want to permanently delete this file AND its backup?"
                        .to_string(),
                    help: file_path.cloned(),
                    ..Default::default()
                }) {
                    Ok(true) => {
                        debug!("hioi");
                        remove()
                    }
                    _ => 1,
                }
            } else {
                //Remove without confirmation:
                remove()
            }
        }
        Some(("ls", sub_matches)) => {
            let output_as_json = sub_matches.get_flag("json");
            if let Some(backup_name) = sub_matches.get_one::<String>("BACKUP_NAME") {
                match list_backup_files(backup_name, output_as_json) {
                    Err(e) => {
                        eprintln!("{e}");
                        1
                    }
                    _ => 0,
                }
            } else {
                match list_backups(output_as_json) {
                    Err(e) => {
                        eprintln!("{e}");
                        1
                    }
                    _ => 0,
                }
            }
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

    eprintln!("");
    std::process::exit(exit_code);
}
