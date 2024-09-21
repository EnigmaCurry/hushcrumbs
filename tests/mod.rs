mod common;
use common::*;

#[test]
fn check_working_directory_is_temporary() {
    let context = TestBed::new();
    let temp_dir = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
    let working_dir = context.get_working_dir();
    assert!(
        working_dir.starts_with(&temp_dir),
        "Working directory ({working_dir}) is not in the system known TEMPDIR ({temp_dir})"
    );
    let mut pwd = context.shell("pwd");
    pwd.assert().success().stdout(contains(working_dir));
}
