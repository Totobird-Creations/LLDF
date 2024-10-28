rm ./target/wasm32-unknown-unknown/release/examples/*
RUSTFLAGS='--emit=llvm-ir' cargo build --example hello_world --target=wasm32-unknown-unknown --release