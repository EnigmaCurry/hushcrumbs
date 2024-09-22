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
    let mut context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.shell("touch hello.txt").assert().success();
    context.shell("touch bonjour.txt").assert().success();
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
}

#[test]
fn test_file_remove() {
    let mut context = TestBed::new();

    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.shell("touch hello.txt").assert().success();
    context.shell("touch bonjour.txt").assert().success();
    context.shell("touch howdy.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.run("add test hello.txt").assert().success();
    context.run("add test bonjour.txt").assert().success();
    context.run("add test howdy.txt").assert().success();

    context.run("rm test hi.txt").assert().success();
    context
        .run("rm test bonjour.txt --delete --no-confirm")
        .assert()
        .success();
    context.run("rm test howdy.txt").assert().success();

    let hi = &format!("{}/hi.txt", context.temp_dir_path);
    let hello = &format!("{}/hello.txt", context.temp_dir_path);
    let bonjour = &format!("{}/bonjour.txt", context.temp_dir_path);
    let howdy = &format!("{}/howdy.txt", context.temp_dir_path);

    assert_command_output_equals_json(
        &mut context.binary,
        "ls test --json",
        serde_json::json!({
            "backup_name": "test",
            "files": [
                // Insertion order matters!
                hello
             ]
        }),
    );

    assert_path_not_exists(bonjour); // This file is permanently deleted.
    assert_regular_file_exists(hi); // This file is restored, no longer backed up.
    assert_path_is_symlink(hello); // This file is still backed up
    assert_regular_file_exists(howdy); // This file is restored, no longer backed up.
}

#[test]
fn test_restore() {
    let context = TestBed::new();

    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.shell("touch hello.txt").assert().success();
    context.shell("touch bonjour.txt").assert().success();
    context.shell("touch howdy.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.run("add test hello.txt").assert().success();
    context.run("add test bonjour.txt").assert().success();
    context.run("add test howdy.txt").assert().success();
    context
        .shell("rm -f hi.txt hello.txt bonjour.txt howdy.txt")
        .assert()
        .success();

    let hi = &format!("{}/hi.txt", context.temp_dir_path);
    let hello = &format!("{}/hello.txt", context.temp_dir_path);
    let bonjour = &format!("{}/bonjour.txt", context.temp_dir_path);
    let howdy = &format!("{}/howdy.txt", context.temp_dir_path);

    // These files have been deleted, but are still backed up:
    assert_path_not_exists(hi);
    assert_path_not_exists(hello);
    assert_path_not_exists(bonjour);
    assert_path_not_exists(howdy);

    context.run("restore test").assert().success();

    assert_path_is_symlink(hi);
    assert_path_is_symlink(hello);
    assert_path_is_symlink(bonjour);
    assert_path_is_symlink(howdy);
}
