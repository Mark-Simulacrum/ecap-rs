extern crate cc;

fn main() {
    println!("cargo:rustc-link-lib=dylib=ecap");

    cc::Build::new()
        .file("src/shim.cpp")
        .define("PACKAGE_NAME", "\"rcap\"")
        .define("PACKAGE_VERSION", "\"0.0.1\"")
        .define("LIBECAP_VERSION", "\"1.0.1\"")
        .shared_flag(true)
        .static_flag(true)
        .cpp(true)
        .warnings(true)
        .extra_warnings(true)
        .compile("shim");
}
