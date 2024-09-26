mod common;
use common::*;

#[test]
fn test_completions() {
    let context = TestBed::new();
    context
        .run("completions")
        .assert()
        .failure()
        .stderr(contains("Instructions to enable tab completion"));
    context.run("completions bash").assert().success();
    context.run("completions fish").assert().success();
    context.run("completions zsh").assert().success();
}
