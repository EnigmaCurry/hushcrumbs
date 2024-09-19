mod common;
use common::*;

#[test]
fn check_temporary_directory() {
    let context = TestBed::new();
    let mut command = Command::new("pwd");
    command.current_dir(context.temp_dir.path());
    command
        .assert()
        .success()
        .stdout(contains(context.temp_dir.path().to_str().unwrap()));
}
