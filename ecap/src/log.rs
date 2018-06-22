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
