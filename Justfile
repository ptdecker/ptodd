# Build web site
build:
    cargo fmt
    cargo clippy
    cargo build

# Run the web site from source
run:
    cargo run
