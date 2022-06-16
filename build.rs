fn main() {
    println!("cargo:rustc-link-arg=--no-entry");
    
    // THIS DOES NOT WORK, because something sets in CLI it already:
    // println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");

    // THIS WORKS!
    println!("cargo:rustc-env=EMCC_CFLAGS=-s ERROR_ON_UNDEFINED_SYMBOLS=0");
    
    cc::Build::new()
        .file("src/test.c")
        .compile("test");
}