mod common;
use common::*;

#[test]
fn test_file_add() {
    let mut context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.shell("touch hello.txt").assert().success();
    context.shell("touch bonjour.txt").assert().success();
    context.shell("touch other.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.run("add test hello.txt").assert().success();
    context.run("add test bonjour.txt").assert().success();

    assert_command_output_equals_json(
        &mut context.binary,
        "ls test --json",
        serde_json::json!({
            "backup_name": "test",
            "files": [
                // Insertion order matters!
                format!("{}/hi.txt", context.temp_dir_path),
                format!("{}/hello.txt", context.temp_dir_path),
                format!("{}/bonjour.txt", context.temp_dir_path),
             ]
        }),
    );

    // add the same file again and it should error:
    context.run("add test bonjour.txt").assert().failure();
    // add a non-backup symlink, it should error:
    context.shell("ln -s other.txt link.txt").assert().success();
    context.run("add test link.txt").assert().failure();
}

#[test]
fn test_file_add_but_paths_file_is_corrupt() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context
        .shell("echo bad data > t/paths.ron")
        .assert()
        .success();
    context.shell("touch hello.txt").assert().success();
    context
        .run("add test hello.txt")
        .assert()
        .failure()
        .stderr(contains("Failed to parse paths file"));
}
