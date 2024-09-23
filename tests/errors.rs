mod common;
use common::*;

#[test]
fn test_malformed_config() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("ls -l").assert().success();
    context
        .shell("bash -c 'echo \"bad data\" > config.ron'")
        .assert()
        .success();
    context.shell("cat config.ron").assert().success();
    context
        .run("list test")
        .assert()
        .failure()
        .stderr(contains("Failed to parse config"));
}

#[test]
fn test_restore_but_paths_file_missing() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.shell("rm t/paths.ron").assert().success();
    context
        .run("restore test")
        .assert()
        .failure()
        .stderr(contains("paths.ron is missing"));
}

#[test]
fn test_init_duplicate() {
    let context = TestBed::new();
    // First init is cleanly created:
    context.run("init test t").assert().success();
    // Try init duplicate backup name:
    context.run("init test t2").assert().failure();
    // Try ini duplicate backup dir:
    context.run("init test2 t").assert().failure();
}
