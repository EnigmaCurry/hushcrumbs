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
    context.run("init test t2").assert().failure();
    // Try ini duplicate backup dir:
    context.run("init test2 t").assert().failure();
}
