mod common;
use common::*;

#[test]
fn test_init() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("test -d t").assert().success();
}

#[test]
fn test_init_duplicate() {
    let context = TestBed::new();
    // First init is cleanly created:
    context.run("init test t").assert().success();
    // Try init duplicate backup name:
    context.run("init test t2 -v").assert().failure();
    // Try ini duplicate backup dir:
    context.run("init test2 t").assert().failure();
}

#[test]
fn test_init_but_config_unreadable() {
    let context = TestBed::new();
    context
        .shell("touch config.ron && chmod -r config.ron")
        .assert()
        .success();
    context
        .run("init test t")
        .assert()
        .failure()
        .stderr(contains("config.ron has invalid permissions."));
}

#[test]
fn test_init_but_config_unwritable() {
    let context = TestBed::new();
    context
        .shell("touch config.ron && chmod -w config.ron")
        .assert()
        .success();
    context
        .run("init test t")
        .assert()
        .failure()
        .stderr(contains("config.ron has invalid permissions."));
}
