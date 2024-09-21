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

bin-deps:
    cargo binstall --no-confirm cargo-nextest

[no-cd]
run *args:
    cargo run --manifest-path "${current_dir}/Cargo.toml" -- {{args}}

build *args:
    cargo build {{args}}

build-watch *args:
    cargo watch -s "clear && cargo build {{args}}"

test *args:
    cargo nextest run --release -- {{args}}

test-watch *args:
    cargo watch -s "clear && cargo nextest run --release -- {{args}}"

test-verbose *args:
    RUST_TEST_THREADS=1 cargo nextest run --nocapture --release -- {{args}}

test-watch-verbose *args:
    RUST_TEST_THREADS=1 cargo watch -s "clear && cargo nextest run --nocapture --release -- {{args}}"
    
clippy *args:
    RUSTFLAGS="-D warnings" cargo clippy {{args}} --color=always 2>&1 --tests | less -R

