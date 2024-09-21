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
#[allow(dead_code)]
impl TestBed {
    fn get_binary(working_dir: &TempDir) -> Command {
        let mut binary = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Binary not found");
        binary.current_dir(working_dir.path());
        binary.arg("-c config.ron");
        binary
    }
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        info!(
            "Created temporary directory: {}",
            temp_dir.path().to_str().unwrap()
        );
        let binary = Self::get_binary(&temp_dir);
        Self { temp_dir, binary }
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
