cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/melt.wasm typst_package/
cp README.md typst_package\
