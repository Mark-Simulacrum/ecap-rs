use libc::size_t;

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

#[repr(C)]
pub struct RustLogVerbosity(size_t);

impl RustLogVerbosity {
    pub fn new() -> RustLogVerbosity {
        RustLogVerbosity(
            ImportanceLevel::Critical as usize |
            (FrequencyLevel::Operation as usize) << 8 |
            (MessageSizeLevel::Normal as usize) << 16
        )
    }
}
