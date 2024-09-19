use assert_cmd::Command;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use predicates::str::contains;
use tempfile::TempDir;

#[ctor::ctor]
fn setup_logging() {
    env_logger::builder().is_test(true).try_init().unwrap();
}

pub struct TestBed {
    pub temp_dir: TempDir,
    pub binary: Command,
}
impl TestBed {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let binary = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Binary not found");
        Self { temp_dir, binary }
    }
}

#[test]
fn test_cli_help() {
    let mut context = TestBed::new();
    context
        .binary
        .current_dir(context.temp_dir.path())
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Usage"));
}
