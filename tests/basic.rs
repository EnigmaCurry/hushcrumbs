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

    context.run("rm test unknown.txt").assert().failure();

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

    // Test restore
    context.run("restore test").assert().success();
    assert_path_is_symlink(hi);
    assert_path_is_symlink(hello);
    assert_path_is_symlink(bonjour);
    assert_path_is_symlink(howdy);

    // Restoring is idempotent
    context.run("restore test").assert().success();
    assert_path_is_symlink(hi);
    assert_path_is_symlink(hello);
    assert_path_is_symlink(bonjour);
    assert_path_is_symlink(howdy);

    // You can also restore by copying instead of symlinking:
    context.shell("rm hi.txt").assert().success();
    context.run("restore test --copy").assert().success();
    assert_regular_file_exists(hi);
    assert_path_is_symlink(hello);
    assert_path_is_symlink(bonjour);
    assert_path_is_symlink(howdy);
}

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
fn test_deinit() {
    let mut context = TestBed::new();
    context.run("init one t1").assert().success();
    context.run("init two t2").assert().success();
    context.run("init three t3").assert().success();

    context.shell("test -d t1").assert().success();
    context.shell("test -d t2").assert().success();
    context.shell("test -d t3").assert().success();

    context.run("deinit one").assert().success();
    context.run("deinit two").assert().success();

    assert_command_output_equals_json(
        &mut context.binary,
        "ls --json",
        serde_json::json!({
            "backups": [
                {"name": "three", "path": context.temp_dir.path().join("t3").display().to_string()},
             ]
        }),
    );

    // Add files to the backup, and this should prevent deinit:
    context.shell("touch test.txt").assert().success();
    context.run("add three test.txt").assert().success();
    context.run("deinit three").assert().failure();

    // Remove the file and deinit should now work:
    context.run("rm three test.txt").assert().success();
    context.run("deinit three").assert().success();
}
