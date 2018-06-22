#![allow(unused)]
use libc::{size_t, c_char, c_void, c_int};
use std::ops;
use ffi;

use ecap::{Area, PascalStr, Name};

#[derive(Debug, Copy, Clone, Eq)]
pub struct Version {
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub micro: Option<u32>,
}

impl Version {
    pub fn from_raw(v: ffi::Version) -> Self {
        Version {
            major: if v.major >= 0 { Some(v.major as u32) } else { None },
            minor: if v.minor >= 0 { Some(v.minor as u32) } else { None },
            micro: if v.micro >= 0 { Some(v.micro as u32) } else { None },
        }
    }

    pub fn raw(self) -> ffi::Version {
        ffi::Version {
            major: self.major.map_or(-1, |v| v as c_int),
            minor: self.minor.map_or(-1, |v| v as c_int),
            micro: self.micro.map_or(-1, |v| v as c_int),
        }
    }

    pub fn known(&self) -> bool {
        self.major.is_some()
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.known() &&
        self.major == other.major &&
        self.minor == other.minor &&
        self.micro == other.micro
    }
}

foreign_ref!(pub struct FirstLine(ffi::FirstLine));

impl FirstLine {
    pub fn version(&self) -> Version {
        unsafe {
            Version::from_raw(ffi::rust_shim_version(self.as_ptr()))
        }
    }

    pub fn set_version(&mut self, version: Version) {
        unsafe {
            ffi::rust_shim_set_version(self.as_ptr_mut(), &version.raw())
        }
    }

    pub fn protocol(&self) -> Name {
        unimplemented!()
    }

    pub fn set_protocol(&mut self, protocol: Name) {
        unimplemented!()
    }
}

pub struct RequestLine(*mut ffi::RequestLine);

// FIXME: deref to FirstLine
impl RequestLine {
    pub fn uri(&self) -> Area {
        unimplemented!()
    }

    pub fn set_uri(&mut self, area: &Area) {
        unimplemented!()
    }

    pub fn method(&self) -> Name {
        unimplemented!()
    }

    pub fn set_method(&mut self, area: &Name) {
        unimplemented!()
    }
}

pub struct StatusLine(*mut ffi::StatusLine);

// deref to FirstLine
impl StatusLine {
    pub fn status_code(&self) -> c_int {
        unimplemented!()
    }

    pub fn set_status_code(&mut self, code: c_int) {
        unimplemented!()
    }

    pub fn reason_phrase(&self) -> Area {
        unimplemented!()
    }

    pub fn set_reason_phrase(&mut self, area: &Area) {
        unimplemented!()
    }
}

#[repr(transparent)]
pub struct SharedPtrMessage(ffi::SharedPtrMessage);

impl SharedPtrMessage {
    pub fn as_ptr(&self) -> *const ffi::SharedPtrMessage {
        &self.0 as *const _
    }
}

impl ops::Deref for SharedPtrMessage {
    type Target = Message;
    fn deref(&self) -> &Message {
        unsafe {
            Message::from_ptr(ffi::rust_shim_shared_ptr_message_ref(&self.0 as *const _))
        }
    }
}

impl ops::DerefMut for SharedPtrMessage {
    fn deref_mut(&mut self) -> &mut Message {
        unsafe {
            Message::from_ptr_mut(ffi::rust_shim_shared_ptr_message_ref_mut(&mut self.0 as *mut _))
        }
    }
}

impl Drop for SharedPtrMessage {
    fn drop(&mut self) {
        unsafe {
            ffi::rust_shim_shared_ptr_message_free(&mut self.0 as *mut _);
        }
    }
}

foreign_ref!(pub struct Message(ffi::Message));

impl Message {
    pub fn clone(&self) -> SharedPtrMessage {
        unsafe {
            SharedPtrMessage(ffi::rust_shim_message_clone(self.as_ptr()))
        }
    }

    accessor!(fn first_line() -> &FirstLine: ffi::rust_shim_message_first_line);
    accessor!(fn first_line_mut() -> &mut FirstLine: ffi::rust_shim_message_first_line_mut);
    accessor!(fn header() -> &Header: ffi::rust_shim_message_header);
    accessor!(fn header_mut() -> &mut Header: ffi::rust_shim_message_header_mut);

    pub fn body(&self) -> Option<&Body> {
        unsafe {
            let ptr = ffi::rust_shim_message_body(self.as_ptr());
            if ptr.is_null() {
                return None;
            }
            Some(Body::from_ptr(ptr))
        }
    }

    pub fn body_mut(&mut self) -> Option<&mut Body> {
        unsafe {
            let ptr = ffi::rust_shim_message_body_mut(self.as_ptr_mut());
            if ptr.is_null() {
                return None;
            }
            Some(Body::from_ptr_mut(ptr))
        }
    }
}

foreign_ref!(pub struct Body(ffi::Body));

impl Body {
    pub fn size(&self) -> Option<u64> {
        unsafe {
            let size = ffi::rust_shim_body_size(self.as_ptr());
            if size.known {
                Some(size.size)
            } else {
                None
            }
        }
    }
}

foreign_ref!(pub struct Header(ffi::Header));

impl Header {
    pub fn has_any(&self, name: &Name) -> bool {
        unsafe {
            ffi::rust_shim_header_has_any(self.as_ptr(), name.as_ptr())
        }
    }

    pub fn value(&self, name: &Name) -> Area {
        unsafe {
            Area::from_raw(ffi::rust_shim_header_value(self.as_ptr(), name.as_ptr()))
        }
    }

    pub fn add(&mut self, name: &Name, value: &Area) {
        unsafe {
            ffi::rust_shim_header_add(self.as_ptr_mut(), name.as_ptr(), value.as_ptr())
        }
    }

    pub fn remove_any(&mut self, name: &Name) {
        unsafe {
            ffi::rust_shim_header_remove_any(self.as_ptr_mut(), name.as_ptr())
        }
    }

    pub fn image(&self) -> Area {
        unsafe {
            Area::from_raw(ffi::rust_shim_header_image(self.as_ptr()))
        }
    }

    pub fn parse(&mut self, buf: &Area) {
        unsafe {
            ffi::rust_shim_header_parse(self.as_ptr_mut(), buf.as_ptr())
        }
    }

    pub fn visit_each(&self, callback: fn(&Name, &[u8])) {
        unsafe {
            ffi::rust_shim_header_visit_each(
                self.as_ptr(), ffi::visitor_callback, callback as *const c_void);
        }
    }
}
