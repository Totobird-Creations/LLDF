cd ../lldf-bind &&
../lldf-bind/build.sh &&
cd ../lldf &&
cargo run -- build ../lldf-bind/target/wasm32-unknown-unknown/release/examples/hello_world-e11014e55926950f.ll --iapi
# Basically needs to be changed every recompile. Need to find a better way to automate this.
