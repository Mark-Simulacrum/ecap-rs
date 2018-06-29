use std::fmt;
use std::ptr::NonNull;
use ffi;
use libc::c_char;
use ecap::common::log::{self, LogVerbosity};

impl log::DebugStream for DebugStream { }

pub struct DebugStream(Option<NonNull<Ostream>>);

impl DebugStream {
    pub fn new() -> Self {
        unsafe {
            let verbosity = LogVerbosity::new();
            let ptr = ffi::rust_shim_host_open_debug(ffi::LogVerbosity(verbosity.mask()));
            if ptr.is_null() {
                DebugStream(None)
            } else {
                let ostream = ptr as *mut Ostream;
                DebugStream(Some(NonNull::from(&mut *ostream)))
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
                ffi::rust_shim_host_close_debug(stream.as_mut() as *mut _ as *mut ffi::Ostream);
            }
        }
    }
}

foreign_ref!(pub struct Ostream(ffi::Ostream));

impl fmt::Write for Ostream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            ffi::rust_shim_ostream_write(self as *mut _ as *mut ffi::Ostream, s.as_ptr() as *const c_char, s.len());
        }
        Ok(())
    }
}