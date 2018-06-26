use ffi;
use libc::c_char;
use std::fmt;
use std::ptr::NonNull;

/// Importance of the logged message to the host application admin
pub enum ImportanceLevel {
    /// Debugging information. Not normally logged.
    Debug = 0,

    /// General information. Seen and logged by default.
    Normal = 1,

    /// Information logged and seen in "quiet" mode.
    Critical = 2,
}

/// Quantity of messages expected under normal conditions
pub enum FrequencyLevel {
    /// Many times in transaction lifetime
    Operation = 0,

    /// Once/twice in transaction lifetime
    Xaction = 1 << 4,

    /// Occurs just a few times in application lifetime
    Application = 2 << 4,
}

/// Message length in normal conditions
pub enum MessageSizeLevel {
    /// Regular log line, under ~120 characters
    Normal = 0,

    /// Data dumps mostly
    Large = 1 << 8,
}

pub struct LogVerbosity(ffi::LogVerbosity);

impl LogVerbosity {
    pub fn new() -> LogVerbosity {
        LogVerbosity(ffi::LogVerbosity(
            ImportanceLevel::Critical as usize
                | (FrequencyLevel::Operation as usize) << 8
                | (MessageSizeLevel::Normal as usize) << 16,
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
            ffi::rust_shim_ostream_write(self.as_ptr_mut(), s.as_ptr() as *const c_char, s.len());
        }
        Ok(())
    }
}
