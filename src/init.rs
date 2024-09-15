use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    backups: HashMap<String, String>, // Backup name -> path
}

pub fn init_backup(backup_name: &str, path: Option<&str>) -> io::Result<()> {
    let backup_path = Path::new(path.unwrap_or(".")).join(backup_name);
    if backup_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Backup path already exists",
        ));
    }

    create_dir(&backup_path)?;
    update_config(backup_name, &backup_path)?;
    Ok(())
}

pub fn update_config(backup_name: &str, path: &Path) -> io::Result<()> {
    let config_path = dirs::config_dir().unwrap().join("secrets/config.ron");
    let mut config: Config = if config_path.exists() {
        ron::de::from_reader(File::open(&config_path)?).unwrap_or_default()
    } else {
        Config {
            backups: HashMap::new(),
        }
    };

    config
        .backups
        .insert(backup_name.to_string(), path.to_str().unwrap().to_string());

    let mut file = File::create(config_path)?;
    write!(file, "{}", ron::ser::to_string(&config).unwrap())?;
    Ok(())
}
