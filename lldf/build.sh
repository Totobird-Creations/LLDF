sh ./setenv.sh

rm ./rustc-ice-*.txt
# Build the `hello_world` example in `lldf-bind`.
cd ../lldf-bind && ./build.sh && cd ../lldf &&
# Find the location of the emitted llvm ir file.
path=$(ls -l ../lldf-bind/target/wasm32-unknown-unknown/release/examples/*.ll | awk '{print $9}') &&
# Build the template.
cargo run -- build $path --ccapi
