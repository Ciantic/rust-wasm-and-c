fn main() {
    println!("cargo:rustc-link-arg=-s");
    println!("cargo:rustc-link-arg=ERROR_ON_UNDEFINED_SYMBOLS=0");
    println!("cargo:rustc-link-arg=--no-entry");
    // println!("cargo:rustc-link-arg=-sLLD_REPORT_UNDEFINED");
    // println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");
    
    cc::Build::new()
        .file("src/test.c")
        .compile("test");
}