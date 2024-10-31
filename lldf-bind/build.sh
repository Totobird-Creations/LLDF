rm ./target/wasm32-unknown-unknown/release/examples/*
RUSTFLAGS='--emit=llvm-ir -Zlocation-detail=none -Zfmt-debug=none' cargo build --example hello_world --target=wasm32-unknown-unknown --release