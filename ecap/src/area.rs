use libc::c_char;
use std::{ptr, mem, slice, fmt};

use ffi;

pub struct Area(ffi::Area);

impl Area {
    pub fn new() -> Area {
        unsafe {
            Area(ffi::rust_area_new())
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.0.buf as *mut u8, self.0.size)
        }
    }

    pub fn as_ptr(&self) -> *const ffi::Area {
        &self.0 as *const _
    }

    pub fn as_ptr_mut(&mut self) -> *mut ffi::Area {
        &mut self.0 as *mut _
    }

    pub fn raw(self) -> ffi::Area {
        // This is needed to bypass the destructor of Area; when we pass it to FFI via
        // `ffi::Area` we're presuming that C++ will pass it back or run the d-tor itself
        unsafe {
            let raw = ptr::read(self.as_ptr());
            mem::forget(self);
            raw
        }
    }

    pub fn from_raw(r: ffi::Area) -> Area {
        Area(r)
    }
}

impl Drop for Area {
    fn drop(&mut self) {
        unsafe {
            ffi::rust_area_free(self.as_ptr_mut());
        }
    }
}

impl<T: AsRef<[u8]>> From<T> for Area {
    fn from(v: T) -> Self {
        let data = v.as_ref();
        unsafe {
            // FIXME: Avoid the copy here
            Area(ffi::rust_area_new_slice(data.as_ptr() as *const c_char, data.len()))
        }
    }
}

impl fmt::Debug for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", String::from_utf8_lossy(self.as_bytes()))
    }
}
