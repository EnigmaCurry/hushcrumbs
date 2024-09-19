set export

current_dir := `pwd`
RUST_LOG := "debug"
RUST_BACKTRACE := "1"

deps:
    @echo
    @echo "Installing dependencies:"
    @echo
    cargo install cargo-nextest
    @echo
    @echo "All dependencies have been installed."
    @echo
    @echo 'Type `just run` to build and run the development binary, and specify any args after that.'
    @echo 'For example: `just run help`'
    @echo

[no-cd]
run *args:
    cargo run --manifest-path "${current_dir}/Cargo.toml" -- {{args}}

build:
    cargo build

test:
    cargo nextest run --release

test-verbose:
    RUST_TEST_THREADS=1 cargo test --release  -- --nocapture

clippy:
    RUSTFLAGS="-D warnings" cargo clippy --color=always 2>&1 --tests | less -R

clippy-fix:
    RUSTFLAGS="-D warnings" cargo clippy --fix --color=always 2>&1 --tests | less -R
