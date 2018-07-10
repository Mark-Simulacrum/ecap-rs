use ecap::common::Version;
use ffi;
use libc::c_int;

pub struct CppVersion;

impl CppVersion {
    pub fn from_raw(v: ffi::Version) -> Version {
        let major = if v.major == -1 {
            None
        } else {
            Some(v.major as u32)
        };
        let minor = if v.minor == -1 {
            None
        } else {
            Some(v.minor as u32)
        };
        let micro = if v.micro == -1 {
            None
        } else {
            Some(v.micro as u32)
        };
        Version {
            major,
            minor,
            micro,
        }
    }

    pub fn to_raw(v: Version) -> ffi::Version {
        ffi::Version {
            major: v.major.map(|v| v as c_int).unwrap_or(-1),
            minor: v.minor.map(|v| v as c_int).unwrap_or(-1),
            micro: v.micro.map(|v| v as c_int).unwrap_or(-1),
        }
    }
}
