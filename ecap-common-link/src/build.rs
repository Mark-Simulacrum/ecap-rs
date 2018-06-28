pub fn main() {
    println!("cargo:rerun-if-changed=src/build.rs");
    println!("cargo:rustc-link-lib=dylib=ecap_common");
}
