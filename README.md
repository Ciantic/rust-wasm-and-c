# Hello world with WASM, C and Rust

## Emscripten target

```bash
# dependency emscripten and clang...
apt install llvm-dev libclang-dev clang
git clone https://github.com/emscripten-core/emsdk ~/emsdk

# "tot" (for tree top, because emsdk had some issues with emscripten_memcpy_big)
# "latest" is used normally
~/emsdk install tot
~/emsdk activate tot
source ~/emsdk/emsdk_env.sh

rustup target add wasm32-unknown-emscripten
cargo build --target wasm32-unknown-emscripten
```

## Wasi target

TODO: Does not work yet!