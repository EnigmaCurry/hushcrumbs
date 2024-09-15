use add::add_to_backup;
use clap::{Arg, Command};
use init::init_backup;
use list::{list_backup_files, list_backups};
use restore::{remove_from_backup, restore_backup};

mod add;
mod config;
mod init;
mod list;
mod paths;
mod restore;

fn main() {
    let matches = Command::new("secrets")
        .version("1.0")
        .author("Author Name <email@example.com>")
        .about("A CLI backup tool")
        .subcommand(
            Command::new("init")
                .about("Creates a new backup directory")
                .arg(Arg::new("BACKUP_NAME").required(true))
                .arg(Arg::new("PATH").required(false)),
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
            Command::new("remove")
                .about("Removes a file from the backup")
                .arg(Arg::new("BACKUP_NAME").required(true))
                .arg(Arg::new("PATH").required(false))
                .arg(Arg::new("delete").long("delete")),
        )
        .subcommand(
            Command::new("list")
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
        )
        .get_matches();

    // Handle the subcommands and arguments here
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            let path = sub_matches.get_one::<String>("PATH");
            match init_backup(backup_name, path.map(|s| s.as_str())) {
                Ok(_) => println!("Backup '{}' initialized successfully.", backup_name),
                Err(e) => eprintln!("Error initializing backup: {}", e),
            }
        }
        Some(("add", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            let file_path = sub_matches.get_one::<String>("PATH").unwrap();
            match add_to_backup(backup_name, file_path) {
                Ok(_) => println!("File '{}' added to backup '{}'.", file_path, backup_name),
                Err(e) => eprintln!("Error adding file to backup: {}", e),
            }
        }
        Some(("restore", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            let copy = sub_matches.contains_id("copy");
            let overwrite = sub_matches.contains_id("overwrite");
            match restore_backup(backup_name, copy, overwrite) {
                Ok(_) => println!("Backup '{}' restored successfully.", backup_name),
                Err(e) => eprintln!("Error restoring backup: {}", e),
            }
        }
        Some(("remove", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            let file_path = sub_matches.get_one::<String>("PATH");
            let delete = sub_matches.contains_id("delete");
            match remove_from_backup(backup_name, file_path.map(|s| s.as_str()), delete) {
                Ok(_) => println!("File removed from backup '{}'.", backup_name),
                Err(e) => eprintln!("Error removing file from backup: {}", e),
            }
        }
        Some(("list", sub_matches)) => {
            if let Some(backup_name) = sub_matches.get_one::<String>("BACKUP_NAME") {
                match list_backup_files(backup_name) {
                    Ok(files) => {
                        println!("Files in backup '{}':", backup_name);
                        for file in files {
                            println!("{}", file);
                        }
                    }
                    Err(e) => eprintln!("Error listing files in backup: {}", e),
                }
            } else {
                match list_backups() {
                    Ok(backups) => {
                        println!("Available backups:");
                        for backup in backups {
                            println!("{}", backup);
                        }
                    }
                    Err(e) => eprintln!("Error listing backups: {}", e),
                }
            }
        }
        Some(("commit", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            println!("Commit backup '{}' (not implemented).", backup_name);
        }
        Some(("push", sub_matches)) => {
            let backup_name = sub_matches.get_one::<String>("BACKUP_NAME").unwrap();
            println!("Push backup '{}' (not implemented).", backup_name);
        }
        _ => {
            eprintln!("Unknown subcommand or missing arguments. Use --help for more information.");
        }
    }
}
