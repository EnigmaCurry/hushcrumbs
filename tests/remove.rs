mod common;
use common::*;

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

    //context.shell("ls -l").assert().failure();
    let hi = &format!("{}/hi.txt", context.temp_dir_path);
    let hello = &format!("{}/hello.txt", context.temp_dir_path);
    let bonjour = &format!("{}/bonjour.txt", context.temp_dir_path);
    let howdy = &format!("{}/howdy.txt", context.temp_dir_path);
    assert_path_is_symlink(&hi);
    assert_path_is_symlink(&hello);
    assert_path_is_symlink(&bonjour);
    assert_path_is_symlink(&howdy);
    let hi_backup = canonicalize(hi).unwrap();
    let hello_backup = canonicalize(hello).unwrap();
    let bonjour_backup = canonicalize(bonjour).unwrap();
    let howdy_backup = canonicalize(howdy).unwrap();
    assert_regular_file_exists(hi_backup.to_str().unwrap());
    assert_regular_file_exists(hello_backup.to_str().unwrap());
    assert_regular_file_exists(bonjour_backup.to_str().unwrap());
    assert_regular_file_exists(howdy_backup.to_str().unwrap());

    // Remove files:
    context.run("rm test hi.txt").assert().success();
    assert_regular_file_exists(hi); // This file is restored, no longer backed up.
    assert_path_not_exists(hi_backup.to_str().unwrap()); // The backup is removed.

    context
        .run("rm test bonjour.txt --delete --no-confirm")
        .assert()
        .success();
    assert_path_not_exists(bonjour); // This file is permanently deleted.
    assert_path_not_exists(bonjour_backup.to_str().unwrap()); // The backup is removed.

    // Manually remove the howdy.txt symlink, and then test removing it from the backup:
    context.shell("rm howdy.txt").assert().success();
    // This should not work, because the existing path does not exist.
    context
        .run("rm test howdy.txt")
        .assert()
        .failure()
        .stderr(contains("The existing path does not exist"));
    assert_path_not_exists(howdy); // This file still does not exist;
    assert_regular_file_exists(howdy_backup.to_str().unwrap()); // But the backup still does.

    // Use the --delete to really delete the backup even though the path does not exist:
    context
        .run("rm test howdy.txt --delete --no-confirm")
        .assert()
        .success();
    assert_path_not_exists(howdy_backup.to_str().unwrap()); // The backup is removed.

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

    // Test removing an unknown file:
    context.run("rm test unknown.txt").assert().failure();

    // Permanently remove hello :
    assert_path_is_symlink(hello); // This file is still backed up
    context
        .run("rm test hello.txt --delete --no-confirm")
        .assert()
        .success();
    assert_path_not_exists(hello); // This file is permanently deleted.
    assert_path_not_exists(hello_backup.to_str().unwrap()); // And the backup
}

#[test]
fn test_remove_conflict() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context
        .shell("rm hi.txt && touch hi.txt")
        .assert()
        .success();
    // without the --no-confirm flag, this fails because the overwrite prompt
    // is non-interactive:
    context
        .run("remove test hi.txt")
        .assert()
        .failure()
        .stderr(contains(
            "A conflicting non-backup file exists in the original path",
        ));
    // with the --delete flag, the file will be overwritten:
    context
        .run("remove test hi.txt --delete --no-confirm")
        .assert()
        .success();
}

#[test]
fn test_remove_but_paths_file_missing() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.shell("rm t/paths.ron").assert().success();
    context
        .run("remove test hi.txt")
        .assert()
        .failure()
        .stderr(contains("paths.ron is missing"));
}
