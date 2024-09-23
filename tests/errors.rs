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
