set export

current_dir := `pwd`
RUST_LOG := "debug"
RUST_BACKTRACE := "1"

[no-cd]
run *args:
    cargo run -- {{args}}

build:
    cargo build

test:
    cargo test --release

test-verbose:
    RUST_TEST_THREADS=1 cargo test --release  -- --nocapture

clippy:
    RUSTFLAGS="-D warnings" cargo clippy --color=always 2>&1 --tests | less -R

clippy-fix:
    RUSTFLAGS="-D warnings" cargo clippy --fix --color=always 2>&1 --tests | less -R
