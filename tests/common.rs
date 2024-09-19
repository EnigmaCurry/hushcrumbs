pub use assert_cmd::Command;
#[allow(unused_imports)]
pub use log::{debug, error, info, warn};
pub use predicates::str::contains;
use tempfile::TempDir;

#[ctor::ctor]
fn setup_logging() {
    env_logger::builder().is_test(true).try_init().unwrap();
}

pub struct TestBed {
    #[allow(dead_code)]
    pub temp_dir: TempDir,
    #[allow(dead_code)]
    pub binary: Command,
}
impl TestBed {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let mut binary = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Binary not found");
        binary.current_dir(temp_dir.path());
        info!(
            "Created temporary directory: {}",
            temp_dir.path().to_str().unwrap()
        );
        Self { temp_dir, binary }
    }
}
