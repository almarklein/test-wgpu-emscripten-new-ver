# test-wgpu-emscripten-new-ver

Proof of Concept for using wasm-bindgen with an Emscripten build, which
is supported as of Emscripten v6.0.10


## Install Emscripten SDK somewhere

git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install 6.0.10
./emsdk activate 6.0.10
source ./emsdk_env.sh
emcc --version  # Check


## Now move to project dir

cd project_dir


## Install wasm-bindgen cli with matching version

cargo tree -i wasm-bindgen  # to show version
cargo install -f wasm-bindgen-cli --version x.y.z
wasm-bindgen --version  # check


## Build

cargo update
cargo clean
cargo build --release


## Test

python -m http.server
# open browser at localhost:8000
