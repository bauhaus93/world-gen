RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused_imports" RUST_LOG="app,world_gen=debug" cargo run --bin app --release
