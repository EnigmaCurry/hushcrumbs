current_dir := `pwd`
RUST_LOG := "debug"
RUST_BACKTRACE := "1"

[no-cd]
run *args:
    cargo run -- {{args}}


build:
    cargo build
