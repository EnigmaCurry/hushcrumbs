mod common;
use common::*;

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
