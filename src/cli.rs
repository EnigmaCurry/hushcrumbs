use crate::config;
use clap::{Arg, Command};

pub fn app() -> Command {
    Command::new("hushcrumbs")
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
                .default_value(config::get_default_config_path())
                .help("Sets the path to the global config file")
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
        )
        .subcommand(
            Command::new("completions")
                .about("Generates shell (tab) completions script")
                .arg(
                    Arg::new("shell")
                        .help("The shell to generate completions for")
                        .required(false)
                        .value_parser(["bash","zsh","fish"])
                )
        )
}
