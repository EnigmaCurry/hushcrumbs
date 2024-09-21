mod common;
use common::*;

#[test]
fn test_cli_help() {
    let mut context = TestBed::new();
    context.binary.assert().success().stdout(contains("Usage"));
}

#[test]
fn test_init() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("test -d t").assert().success();
}

#[test]
fn test_file_add() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hii.txt").assert().success();
    context.shell("ls").assert().success();
    context.run("add test hii.txt").assert().success();
}
