use libc::{size_t, c_int, c_char, c_void};
use std::ptr::NonNull;
use std::mem;
use std::str;
use std::slice;
use std::fmt;
use std::ops;
use std::ptr;

use log::RustLogVerbosity;

extern {
    pub type Options;
    pub type Host;
    pub type Ostream;
}

extern "C" {
    fn rust_name_new_unknown() -> Name;
    fn rust_name_new_image(buf: *const c_char, len: size_t) -> Name;
    fn rust_name_new_image_id(buf: *const c_char, len: size_t, id: c_int) -> Name;
    fn rust_name_identified(name: *const Name) -> bool;
    fn rust_name_known(name: *const Name) -> bool;
    fn rust_name_eq(a: *const Name, b: *const Name) -> bool;
    fn rust_name_image(a: *const Name) -> CString;
    fn rust_name_free(name: *mut Name);
}

#[repr(C)]
#[repr(align(8))]
pub struct Name([u8; 40]);

macro_rules! const_assert {
    ($condition:expr) => {
        #[deny(const_err)]
        #[allow(dead_code)]
        const ASSERT: usize = 0 - !$condition as usize;
    }
}

const_assert!(mem::align_of::<Name>() == 8);

// C++ type in actuality
impl !Sync for Name {}

impl Name {
    pub fn unknown() -> Name {
        unsafe {
            rust_name_new_unknown()
        }
    }

    pub fn with_image(image: &[u8]) -> Name {
        unsafe {
            rust_name_new_image(image.as_ptr() as *const c_char, image.len())
        }
    }

    pub fn with_image_and_identified(image: &[u8], id: c_int) -> Name {
        unsafe {
            rust_name_new_image_id(image.as_ptr() as *const c_char, image.len(), id)
        }
    }

    pub fn image<'a>(&'a self) -> &'a PascalStr {
        unsafe {
            let s = rust_name_image(self);
            assert_eq!(s.capacity, 0);
            let bytes: &'a [u8] = &*(s.as_bytes() as *const _);
            PascalStr::new(bytes)
        }
    }

    pub fn known(&self) -> bool {
        unsafe {
            rust_name_known(self)
        }
    }

    pub fn identified(&self) -> bool {
        unsafe {
            rust_name_identified(self)
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
            rust_name_eq(self, other)
        }
    }
}

impl Drop for Name {
    fn drop(&mut self) {
        unsafe {
            rust_name_free(self);
        }
    }
}

pub type VisitorCallback = extern "C" fn(*const Name, *const c_char, size_t, *const c_void);

pub extern "C" fn visitor_callback(
    name: *const Name,
    buf: *const c_char,
    len: size_t,
    cb: *const c_void,
) {
    assert!(!name.is_null());
    assert!(!buf.is_null());
    assert!(!cb.is_null());
    unsafe {
        let value = PascalStr::from_c(buf, len);
        let f: fn(&Name, &PascalStr) = mem::transmute(cb);
        f(&*name, value);
    }
}

extern "C" {
    fn options_option(options: *const Options, buf: *const c_char, len: size_t) -> RustArea;
    fn options_visit(options: *const Options, cb: VisitorCallback, extra: *const c_void);
}

impl Options {
    pub fn option(&self, name: &[u8]) -> RustArea {
        unsafe {
            options_option(self as *const Options, name.as_ptr() as *const _, name.len())
        }
    }
    pub fn visit(&self, callback: fn(&Name, &PascalStr)) {
        unsafe {
            options_visit(self as *const Options, visitor_callback, callback as *const c_void);
        }
    }
}

extern "C" {
    fn rust_host() -> *const Host;
    fn rust_shim_host_uri() -> CString;
    fn rust_shim_host_open_debug(verbosity: RustLogVerbosity) -> *mut Ostream;
    fn rust_shim_host_close_debug(stream: *mut Ostream);

    fn rust_shim_ostream_write(stream: *mut Ostream, buf: *const c_char, len: size_t);
}

impl fmt::Write for Ostream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            rust_shim_ostream_write(
                self as *mut Self, s.as_ptr() as *const c_char, s.len());
        }
        Ok(())
    }
}

pub struct DebugStream(Option<NonNull<Ostream>>);

impl DebugStream {
    pub fn new() -> Self {
        unsafe {
            DebugStream(NonNull::new(rust_shim_host_open_debug(RustLogVerbosity::new())))
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
        if let Some(stream) = self.0.take() {
            unsafe {
                rust_shim_host_close_debug(stream.as_ptr());
            }
        }
    }
}

impl Host {
    pub fn uri() -> Vec<u8> {
        unsafe {
            rust_shim_host_uri().to_rust()
        }
    }

    pub fn get<'a>() -> &'a Host {
        unsafe {
            let p = rust_host();
            assert!(!p.is_null());
            &*p
        }
    }
}

#[repr(C)]
pub struct RustArea {
    size: size_t,
    buf: *mut c_char,
    // Scratch space for the shared_ptr in C++; we assert that 16
    // bytes is sufficient in C++.
    details: [u8; 16],
    __align: [u64; 0],
}

impl RustArea {
    pub fn new() -> RustArea {
        unsafe {
            rust_area_new()
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.buf as *mut u8, self.size)
        }
    }
}

// TODO: Implement a way to create without copying data, should be possible by storing
// vec-like in shared_ptr
impl<T: AsRef<[u8]>> From<T> for RustArea {
    fn from(v: T) -> Self {
        let data = v.as_ref();
        unsafe {
            rust_area_new_slice(data.as_ptr() as *const c_char, data.len())
        }
    }
}

impl fmt::Debug for RustArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", String::from_utf8_lossy(self.as_bytes()))
    }
}

extern "C" {
    fn rust_area_new() -> RustArea;
    fn rust_area_new_slice(buf: *const c_char, len: size_t) -> RustArea;
    fn rust_area_free(area: *mut RustArea);
}

impl Drop for RustArea {
    fn drop(&mut self) {
        unsafe {
            rust_area_free(self);
        }
    }
}

#[repr(C)]
pub struct CString {
    size: size_t,
    buf: *const c_char,
    capacity: size_t,
}

impl fmt::Debug for CString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:?}", self.as_bytes())
        } else {
            write!(f, "{:?}", String::from_utf8_lossy(self.as_bytes()))
        }
    }
}

impl CString {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.buf as *mut u8, self.size)
        }
    }

    pub fn to_rust(self) -> Vec<u8> {
        let ret = unsafe {
            Vec::from_raw_parts(
                self.buf as *mut u8,
                self.size,
                self.capacity
            )
        };
        mem::forget(self);
        ret
    }

    pub fn from_static(s: &'static str) -> CString {
        CString {
            size: s.len(),
            buf: s.as_ptr() as *mut c_char,
            capacity: 0,
        }
    }
}

impl<T: Into<Vec<u8>>> From<T> for CString {
    fn from(v: T) -> CString {
        let data = v.into();
        let ret = CString {
            size: data.len(),
            buf: data.as_ptr() as *mut c_char,
            capacity: data.capacity(),
        };
        mem::forget(data);
        ret
    }
}

impl Drop for CString {
    fn drop(&mut self) {
        // FIXME: use pascalstr for capacity = 0
        if self.capacity > 0 {
            let dummy = Vec::new().into();
            let me = mem::replace(self, dummy);
            mem::drop(me.to_rust());
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_new_string(buf: *const c_char, len: size_t) -> CString {
    unsafe {
        let mut data = Vec::with_capacity(len);
        ptr::copy_nonoverlapping(buf as *const u8, data.as_mut_ptr(), len);
        let v = CString {
            size: data.len(),
            buf: data.as_ptr() as *mut c_char,
            capacity: data.capacity(),
        };
        mem::forget(data);
        v
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_free_string(s: CString) {
    mem::drop(s);
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
