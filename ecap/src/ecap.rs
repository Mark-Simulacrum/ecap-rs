use libc::{size_t, c_int, c_char, c_void};
use std::ptr::{self, NonNull};
use std::str;
use std::slice;
use std::fmt;
use std::ops;
use std::mem;

use log::LogVerbosity;
use shim::Ostream;
use ffi;

foreign_ref!(pub struct Options(ffi::Options));

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

pub struct Name(ffi::Name);

impl Name {
    pub fn as_ptr(&self) -> *const ffi::Name {
        &self.0 as *const _
    }

    pub fn as_ptr_mut(&mut self) -> *mut ffi::Name {
        &mut self.0 as *mut _
    }

    pub fn unknown() -> Name {
        unsafe {
            Name(ffi::rust_name_new_unknown())
        }
    }

    pub fn with_image(image: &[u8]) -> Name {
        unsafe {
            Name(ffi::rust_name_new_image(image.as_ptr() as *const c_char, image.len()))
        }
    }

    pub fn with_image_and_identified(image: &[u8], id: c_int) -> Name {
        unsafe {
            Name(ffi::rust_name_new_image_id(image.as_ptr() as *const c_char, image.len(), id))
        }
    }

    pub fn image<'a>(&'a self) -> &'a PascalStr {
        unsafe {
            let ffi::PStr { size, buf } = ffi::rust_name_image(&self.0);
            let bytes = slice::from_raw_parts(buf as *const u8, size);
            PascalStr::new(bytes)
        }
    }

    pub fn known(&self) -> bool {
        unsafe {
            ffi::rust_name_known(&self.0)
        }
    }

    pub fn identified(&self) -> bool {
        unsafe {
            ffi::rust_name_identified(&self.0)
        }
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Name")
            .field("identified", &self.identified())
            .field("known", &self.known())
            .field("image", &self.image())
            .finish()
    }
}

#[test]
fn name_eq_1() {
    assert_eq!(Name::with_image(b"apple"), Name::with_image(b"apple"));
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ffi::rust_name_eq(self.as_ptr(), other.as_ptr())
        }
    }
}

impl Drop for Name {
    fn drop(&mut self) {
        unsafe {
            ffi::rust_name_free(self.as_ptr_mut());
        }
    }
}

impl Options {
    pub fn option(&self, name: &[u8]) -> Area {
        unsafe {
            Area(ffi::options_option(self.as_ptr(), name.as_ptr() as *const _, name.len()))
        }
    }

    pub fn visit(&self, callback: fn(&ffi::Name, &[u8])) {
        unsafe {
            ffi::options_visit(self.as_ptr(), ffi::visitor_callback, callback as *const c_void);
        }
    }
}

pub struct DebugStream(Option<NonNull<Ostream>>);

impl DebugStream {
    pub fn new() -> Self {
        unsafe {
            let ptr = ffi::rust_shim_host_open_debug(LogVerbosity::new().raw());
            if ptr.is_null() {
                DebugStream(None)
            } else {
                DebugStream(Some(NonNull::from(Ostream::from_ptr(ptr))))
            }
        }
    }
}

impl fmt::Write for DebugStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(mut stream) = self.0 {
            unsafe {
                stream.as_mut().write_str(s)?;
            }
        }
        Ok(())
    }
}

impl Drop for DebugStream {
    fn drop(&mut self) {
        if let Some(mut stream) = self.0.take() {
            unsafe {
                ffi::rust_shim_host_close_debug(stream.as_mut().as_ptr_mut());
            }
        }
    }
}

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
