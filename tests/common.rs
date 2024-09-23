use std::process::Output;

pub use assert_cmd::Command;
#[allow(unused_imports)]
pub use log::{debug, error, info, warn};
pub use predicates::str::contains;
use std::env;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[ctor::ctor]
fn setup_logging() {
    env_logger::builder().is_test(true).try_init().unwrap();
}

pub struct TestBed {
    #[allow(dead_code)]
    pub temp_dir: TempDir,
    #[allow(dead_code)]
    pub temp_dir_path: String,
    #[allow(dead_code)]
    pub binary: Command,
}
#[allow(dead_code)]
impl TestBed {
    fn get_binary(working_dir: &TempDir) -> Command {
        let mut binary = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Binary not found");
        binary.current_dir(working_dir.path());
        binary.args(["-c", "config.ron"]);
        binary
    }
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_dir_path = temp_dir.path().display().to_string();
        info!(
            "Created temporary directory: {}",
            temp_dir.path().to_str().unwrap()
        );
        let binary = Self::get_binary(&temp_dir);

        Self {
            temp_dir,
            temp_dir_path,
            binary,
        }
    }
    pub fn run(&self, args: &str) -> Command {
        let args = shell_words::split(args).expect("Bad args: {args:?}");
        let mut binary = Self::get_binary(&self.temp_dir);
        binary.args(args);
        binary
    }
    pub fn shell(&self, cmd: &str) -> Command {
        let parts = shell_words::split(cmd).expect("Failed to parse command: {cmd}");
        assert!(!parts.is_empty(), "Shell command is blank");
        let mut cmd = Command::new(&parts[0]);
        cmd.current_dir(&self.temp_dir);
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }
        cmd
    }
    pub fn get_working_dir(&self) -> &str {
        self.temp_dir
            .path()
            .to_str()
            .expect("Could not stringify TempDir")
    }
}

#[allow(dead_code)]
pub fn assert_command_output_equals_json(
    binary: &mut Command,
    command: &str,
    expected_json: serde_json::Value,
) {
    // Run the command and capture output as JSON
    let output_json = run_command_and_parse_json(binary, command);

    // Assert that the output matches the expected JSON
    assert_eq!(output_json, expected_json);
}

#[allow(dead_code)]
fn run_command_and_parse_json(binary: &mut Command, command: &str) -> serde_json::Value {
    binary.args(shell_words::split(command).expect("Bad args: {args}"));
    let output: Output = binary.assert().success().get_output().clone();

    // Convert output to string and parse as JSON
    serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).unwrap()
}

#[allow(dead_code)]
pub fn assert_regular_file_exists(file_path: &str) {
    let path = Path::new(file_path);

    // Check if the file exists
    assert!(path.exists(), "File does not exist: {}", file_path);

    // Check if it is a regular file (not a directory or symlink)
    let metadata = fs::symlink_metadata(path).expect("Failed to get metadata");
    assert!(
        metadata.is_file(),
        "File is not a regular file: {}",
        file_path
    );
}

#[allow(dead_code)]
pub fn assert_path_not_exists(path_str: &str) {
    let path = Path::new(path_str);

    // Assert that the path does not exist (file, symlink, or directory)
    assert!(!path.exists(), "Path exists: {}", path_str);
}

#[allow(dead_code)]
pub fn assert_path_is_symlink(path_str: &str) {
    let path = Path::new(path_str);

    // Check if the path exists
    assert!(path.exists(), "Path does not exist: {}", path_str);

    // Use symlink_metadata to retrieve metadata without following symlinks
    let metadata = fs::symlink_metadata(path).expect("Failed to get metadata");

    // Ensure the path is a symlink
    assert!(
        metadata.file_type().is_symlink(),
        "Path is not a symlink: {}",
        path_str
    );
}
