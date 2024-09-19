mod common;
use common::*;

#[test]
fn test_cli_help() {
    let mut context = TestBed::new();
    context.binary.assert().success().stdout(contains("Usage"));
}
