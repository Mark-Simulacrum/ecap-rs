extern crate cc;

fn main() {
    println!("cargo:rustc-link-lib=dylib=ecap");

    cc::Build::new()
        .file("src/shim.cpp")
        .shared_flag(true)
        .static_flag(true)
        .cpp(true)
        .warnings(true)
        .extra_warnings(true)
        .compile("shim");
}
