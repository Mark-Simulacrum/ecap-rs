use std::fmt;

/// Importance of the logged message to the host application admin
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImportanceLevel {
    /// Debugging information. Not normally logged.
    Debug = 0,

    /// General information. Seen and logged by default.
    Normal = 1,

    /// Information logged and seen in "quiet" mode.
    Critical = 2,
}

/// Quantity of messages expected under normal conditions
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FrequencyLevel {
    /// Many times in transaction lifetime
    Operation = 0,

    /// Once/twice in transaction lifetime
    Xaction = 1 << 4,

    /// Occurs just a few times in application lifetime
    Application = 2 << 4,
}

/// Message length in normal conditions
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageSizeLevel {
    /// Regular log line, under ~120 characters
    Normal = 0,

    /// Data dumps mostly
    Large = 1 << 8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogVerbosity {
    pub importance: ImportanceLevel,
    pub frequency: FrequencyLevel,
    pub size: MessageSizeLevel,
}

impl LogVerbosity {
    pub fn new() -> LogVerbosity {
        LogVerbosity {
            importance: ImportanceLevel::Critical,
            frequency: FrequencyLevel::Operation,
            size: MessageSizeLevel::Normal,
        }
    }

    /// XXX: This is quite specific to passing through C.
    pub fn mask(&self) -> usize {
        self.importance as usize | self.frequency as usize | self.size as usize
    }
}

pub trait DebugStream: fmt::Write {}
