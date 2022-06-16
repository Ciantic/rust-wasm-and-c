fn main() {
    // wasm32-unknown-emscripten flags
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32"
        && std::env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "unknown"
        && std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "emscripten"
    {
        println!("cargo:rustc-link-arg=--no-entry");
        println!("cargo:rustc-link-arg=-s EXPORT_ES6=1");
        println!("cargo:rustc-link-arg=-s MODULARIZE=1");
        println!("cargo:rustc-link-arg=-o module.html");

        // THIS DOES NOT WORK, because something sets in CLI it already:
        // println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");

        // THIS WORKS!
        println!("cargo:rustc-env=EMCC_CFLAGS=-s ERROR_ON_UNDEFINED_SYMBOLS=0");
    }

    cc::Build::new().file("src/test.c").compile("test");
}
