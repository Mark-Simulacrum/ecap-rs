ecap-rs
----

This is a compatible rewrite of [libecap] in Rust.

This project has the following goals:
 * Permit adapters written in Rust to be easily used by C++ hosts, such as [Squid].
 * Permit hosts written in Rust to use C++ adapters.

The toplevel crates are as follows:
 * ecap: core crate, defines traits and structs (similar to libecap itself)
 * ecap-common: shared library which provides service/translator registration
 * ecap-common-link: workaround for Cargo, shim over ecap-common so
   that crates don't need build scripts
 * ecap-cpp: translator from C++ to Rust types (currently incomplete)
 * ecap-sys: C API for the C++ libecap library.
 * adapter-minimal: minimal adapter written in Rust

[libecap]: e-cap.org
[Squid]: squid-cache.org

