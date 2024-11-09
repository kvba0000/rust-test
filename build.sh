RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features="optimize_for_size,panic_immediate_abort" \
    --release

upx target/release/rust-test