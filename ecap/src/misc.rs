use ffi;

foreign_ref!(pub struct Host(ffi::Host));

impl Host {
    pub fn uri() -> Vec<u8> {
        unsafe { ffi::rust_shim_host_uri().to_rust() }
    }

    pub fn get<'a>() -> &'a Host {
        unsafe { Host::from_ptr(ffi::rust_host()) }
    }
}
