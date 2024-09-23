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
