use ecap::common::log::{self, LogVerbosity};
use ffi;
use host::CppHost;
use libc::c_char;
use std::fmt;
use std::ptr::NonNull;

use call_ffi_maybe_panic;

impl log::DebugStream for DebugStream {}

pub struct DebugStream {
    stream: *mut ffi::Ostream,
    host: *const ffi::Host,
}

impl DebugStream {
    // XXX: This should technically take &'a CppHost and bind that lifetime to itself,
    // but we need GATs for that.
    pub fn from_host(host: &CppHost, verbosity: LogVerbosity) -> Option<Self> {
        let stream = call_ffi_maybe_panic(|out| unsafe {
            ffi::rust_shim_host_open_debug(host.as_ptr(), ffi::LogVerbosity(verbosity.mask()), out)
        });
        if stream.is_null() {
            None
        } else {
            Some(DebugStream {
                stream: stream,
                host: host.as_ptr(),
            })
        }
    }

    fn as_ptr_mut(&mut self) -> *mut ffi::Ostream {
        self.stream
    }
}

impl fmt::Write for DebugStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            ffi::rust_shim_ostream_write(self.as_ptr_mut(), s.as_ptr() as *const c_char, s.len());
        }
        Ok(())
    }
}

impl Drop for DebugStream {
    fn drop(&mut self) {
        call_ffi_maybe_panic(|_| unsafe { ffi::rust_shim_host_close_debug(self.host, self.stream) })
    }
}

foreign_ref!(pub struct Ostream(ffi::Ostream));

impl fmt::Write for Ostream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            ffi::rust_shim_ostream_write(self.as_ptr_mut(), s.as_ptr() as *const c_char, s.len());
        }
        Ok(())
    }
}
