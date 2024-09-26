mod common;
use common::*;

#[test]
fn test_list_backups() {
    let mut context = TestBed::new();
    // When no backups are found, its a failure:
    context.run("ls").assert().failure();

    // Add some backups:
    context.run("init one t1").assert().success();
    context.run("init two t2").assert().success();
    context.run("init three t3").assert().success();

    assert_command_output_equals_json(
        &mut context.binary,
        "ls --json",
        serde_json::json!({
            "backups": [
                {"name": "one", "path": context.temp_dir.path().join("t1").display().to_string()},
                {"name": "two", "path": context.temp_dir.path().join("t2").display().to_string()},
                {"name": "three", "path": context.temp_dir.path().join("t3").display().to_string()},
             ]
        }),
    );
    let tmpdir = context.temp_dir_path.clone();
    context
        .run("ls")
        .assert()
        .success()
        .stdout(contains(formatdoc! {"
              Backup Name | Backup Path 
              -------------+--------------------
               one         | {tmpdir}/t1 
               two         | {tmpdir}/t2 
               three       | {tmpdir}/t3 
"}));
}

#[test]
fn test_list_files() {
    let mut context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.shell("touch hello.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.run("add test hello.txt").assert().success();

    assert_command_output_equals_json(
        &mut context.binary,
        "ls test --json",
        serde_json::json!({
            "backup_name": "test",
            "files": [format!("{}/hi.txt", context.temp_dir_path),
                      format!("{}/hello.txt", context.temp_dir_path)]
        }),
    );
    let tmpdir = context.temp_dir_path.clone();
    context
        .run("ls test")
        .assert()
        .success()
        .stdout(contains(formatdoc! {"
            Local files contained in backup (test): 
           -----------------------------------------
            {tmpdir}/hi.txt 
            {tmpdir}/hello.txt 
"}));
}

#[test]
fn test_list_files_but_backup_gone() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.shell("rm -rf t").assert().success();

    context
        .run("ls test")
        .assert()
        .failure()
        .stderr(contains("Backup directory is missing"));
}

#[test]
fn test_list_files_but_paths_file_gone() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.shell("rm t/paths.ron").assert().success();

    context
        .run("ls test")
        .assert()
        .failure()
        .stderr(contains("No files found in the backup"));
}

#[test]
fn test_list_files_but_paths_file_corrupt() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context
        .shell("echo bad data > t/paths.ron")
        .assert()
        .success();

    context
        .run("ls test")
        .assert()
        .failure()
        .stderr(contains("Failed to parse paths.ron"));
}

#[test]
fn test_list_files_but_no_files_in_backup() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.shell("touch hello.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.run("add test hello.txt").assert().success();
    context.run("rm test hi.txt").assert().success();
    context.run("rm test hello.txt").assert().success();

    context
        .run("ls test")
        .assert()
        .failure()
        .stderr(contains("No files found in the backup"));
}

#[test]
fn test_list_malformed_config() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("ls -l").assert().success();
    context
        .shell("echo bad data > config.ron")
        .assert()
        .success();
    context.shell("cat config.ron").assert().success();
    context
        .run("list test")
        .assert()
        .failure()
        .stderr(contains("Failed to parse config"));
}
