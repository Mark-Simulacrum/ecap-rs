use libc::{size_t, c_char};
use std::slice;
use std::fmt;
use std::ops;

use ffi;

foreign_ref!(pub struct Host(ffi::Host));

impl Host {
    pub fn uri() -> Vec<u8> {
        unsafe {
            ffi::rust_shim_host_uri().to_rust()
        }
    }

    pub fn get<'a>() -> &'a Host {
        unsafe {
            Host::from_ptr(ffi::rust_host())
        }
    }
}

pub struct PascalStr([u8]);

impl fmt::Debug for PascalStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:?}", self)
        } else {
            write!(f, "{:?}", String::from_utf8_lossy(self))
        }
    }
}

impl ops::Deref for PascalStr {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for PascalStr {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl PascalStr {
    pub fn new(s: &[u8]) -> &PascalStr {
        unsafe {
            &*(s as *const [u8] as *const PascalStr)
        }
    }

    pub unsafe fn from_c<'a>(buf: *const c_char, len: size_t) -> &'a PascalStr {
        &*(slice::from_raw_parts(buf as *const u8, len) as *const [u8] as *const PascalStr)
    }

    pub fn to_c(&self) -> (*const c_char, size_t) {
        (self.as_ptr() as *const c_char, self.len())
    }
}
