rm ./rustc-ice-*.txt
rm ./target/wasm32-unknown-unknown/release/examples/*
RUSTFLAGS='--emit=llvm-ir -Zlocation-detail=none -Zfmt-debug=none' cargo build --example liars_deck --target=wasm32-unknown-unknown --release
