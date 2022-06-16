# Hello world with WASM, C and Rust

## Wasm-pack build with C code

Most modern at the moment, generates usable JS and WASM file.

```bash
apt install llvm-dev libclang-dev clang lld
CC=/usr/bin/clang AR=/usr/bin/llvm-ar wasm-pack build --target web --out-dir=pkg
```


Above wasm-pack approach may hide the errors from you and have hidden memory-corruption errors as a result, unless its using wasm32-wasi, I'd recommend against relying on that. [See this.](https://github.com/rustwasm/team/issues/291#issuecomment-1158082482)



## Wasi target

Generates only WASM File

```bash
# dependency emscripten and clang...
apt install llvm-dev libclang-dev clang lld
CC=/usr/bin/clang AR=/usr/bin/llvm-ar cargo build --target wasm32-wasi
```

## Emscripten target (old, avoid this)

Generates WASM and JS file, but bit outdated (uses emscripten)

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
