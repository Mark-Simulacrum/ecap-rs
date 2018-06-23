use std::ptr::NonNull;
use std::fmt;
use libc::c_char;
use ffi;

pub enum ImportanceLevel {
    Debug = 0,
    Normal = 1,
    Critical = 2,
}

pub enum FrequencyLevel {
    Operation = 0,
    Xaction = 1 << 4,
    Application = 2 << 4,
}

pub enum MessageSizeLevel {
    Normal = 0,
    Large = 1 << 8,
}

pub struct LogVerbosity(ffi::LogVerbosity);

impl LogVerbosity {
    pub fn new() -> LogVerbosity {
        LogVerbosity(ffi::LogVerbosity(
            ImportanceLevel::Critical as usize |
            (FrequencyLevel::Operation as usize) << 8 |
            (MessageSizeLevel::Normal as usize) << 16
        ))
    }

    #[inline]
    pub fn raw(self) -> ffi::LogVerbosity {
        self.0
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

foreign_ref!(pub struct Ostream(ffi::Ostream));

impl fmt::Write for Ostream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            ffi::rust_shim_ostream_write(
                self.as_ptr_mut(), s.as_ptr() as *const c_char, s.len());
        }
        Ok(())
    }
}
