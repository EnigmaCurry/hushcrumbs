mod common;
use common::*;

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
fn test_restore_but_paths_file_corrupt() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context
        .shell("echo bad data > t/paths.ron")
        .assert()
        .success();
    context
        .run("restore test")
        .assert()
        .failure()
        .stderr(contains("Failed to parse paths.ron"));
}

#[test]
fn test_restore_but_paths_file_unreadable() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.shell("chmod 000 t/paths.ron").assert().success();
    context
        .run("restore test")
        .assert()
        .failure()
        .stderr(contains("Permission denied"));
}

#[test]
fn test_restore_but_config_file_missing() {
    let context = TestBed::new();
    context
        .run("restore test")
        .assert()
        .failure()
        .stderr(contains("Backup not found"));
}

#[test]
fn test_restore_but_config_file_corrupt() {
    let context = TestBed::new();
    context
        .shell("echo bad data > config.ron")
        .assert()
        .success();
    context
        .run("restore test")
        .assert()
        .failure()
        .stderr(contains("Failed to parse config"));
}

#[test]
fn test_restore_but_backup_unreadable() {
    let context = TestBed::new();
    context.run("init test t").assert().success();
    context.shell("touch hi.txt").assert().success();
    context.run("add test hi.txt").assert().success();
    context.shell("chmod 000 t/*").assert().success();
    context
        .run("restore test")
        .assert()
        .failure()
        .stderr(contains("Permission denied"));
}
