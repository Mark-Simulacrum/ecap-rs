#![allow(unused)]
use libc::{size_t, c_char, c_void, c_int};
use std::ops;

use ecap::{PascalStr, Name, RustArea};

#[repr(C)]
#[derive(Debug)]
pub struct Version {
    major: c_int,
    minor: c_int,
    micro: c_int,
}

impl Version {
    pub fn known(&self) -> bool {
        self.major >= 0
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

extern {
    pub type Message;
    pub type FirstLine;
    pub type RequestLine;
    pub type StatusLine;
    pub type Body;
    pub type Header;
}

extern "C" {
    fn rust_shim_version(line: *const FirstLine) -> Version;
    fn rust_shim_set_version(line: *mut FirstLine, version: *const Version);
    fn rust_shim_body_size(line: *const Body) -> BodySize;
}

impl FirstLine {
    pub fn version(&self) -> Version {
        unsafe {
            rust_shim_version(self)
        }
    }

    pub fn set_version(&mut self, version: &Version) {
        unsafe {
            rust_shim_set_version(self, version)
        }
    }

    pub fn protocol(&self) -> Name {
        unimplemented!()
    }

    pub fn set_protocol(&mut self, protocol: Name) {
        unimplemented!()
    }
}

// deref to FirstLine
impl RequestLine {
    pub fn uri(&self) -> RustArea {
        unimplemented!()
    }

    pub fn set_uri(&mut self, area: &RustArea) {
        unimplemented!()
    }

    pub fn method(&self) -> Name {
        unimplemented!()
    }

    pub fn set_method(&mut self, area: &Name) {
        unimplemented!()
    }
}

// deref to FirstLine
impl StatusLine {
    pub fn status_code(&self) -> c_int {
        unimplemented!()
    }

    pub fn set_status_code(&mut self, code: c_int) {
        unimplemented!()
    }

    pub fn reason_phrase(&self) -> RustArea {
        unimplemented!()
    }

    pub fn set_reason_phrase(&mut self, area: &RustArea) {
        unimplemented!()
    }
}

extern "C" {
    fn rust_shim_message_first_line(msg: *const Message) -> *const FirstLine;
    fn rust_shim_message_first_line_mut(msg: *mut Message) -> *mut FirstLine;
    fn rust_shim_message_header(msg: *const Message) -> *const Header;
    fn rust_shim_message_header_mut(msg: *mut Message) -> *mut Header;
    fn rust_shim_message_body(msg: *const Message) -> *const Body;
    fn rust_shim_message_body_mut(msg: *mut Message) -> *mut Body;
    fn rust_shim_message_clone(msg: *const Message) -> SharedPtrMessage;

    fn rust_shim_shared_ptr_message_ref(msg: *const SharedPtrMessage) -> *const Message;
    fn rust_shim_shared_ptr_message_ref_mut(msg: *mut SharedPtrMessage) -> *mut Message;
    fn rust_shim_shared_ptr_message_free(msg: *mut SharedPtrMessage);
}

#[repr(C)]
pub struct SharedPtrMessage([u8; 16]);

impl ops::Deref for SharedPtrMessage {
    type Target = Message;
    fn deref(&self) -> &Message {
        unsafe {
            &*rust_shim_shared_ptr_message_ref(self)
        }
    }
}

impl ops::DerefMut for SharedPtrMessage {
    fn deref_mut(&mut self) -> &mut Message {
        unsafe {
            &mut *rust_shim_shared_ptr_message_ref_mut(self)
        }
    }
}

impl Drop for SharedPtrMessage {
    fn drop(&mut self) {
        unsafe {
            rust_shim_shared_ptr_message_free(self);
        }
    }
}

impl Message {
    pub fn clone(&self) -> SharedPtrMessage {
        unsafe {
            rust_shim_message_clone(self)
        }
    }

    pub fn first_line(&self) -> &FirstLine {
        unsafe {
            rust_shim_message_first_line(self).as_ref().unwrap()
        }
    }

    pub fn first_line_mut(&mut self) -> &mut FirstLine {
        unsafe {
            rust_shim_message_first_line_mut(self).as_mut().unwrap()
        }
    }

    pub fn header(&self) -> &Header {
        unsafe {
            rust_shim_message_header(self).as_ref().unwrap()
        }
    }

    pub fn header_mut(&mut self) -> &mut Header {
        unsafe {
            rust_shim_message_header_mut(self).as_mut().unwrap()
        }
    }

    pub fn body(&self) -> Option<&Body> {
        unsafe {
            rust_shim_message_body(self).as_ref()
        }
    }

    pub fn body_mut(&mut self) -> Option<&mut Body> {
        unsafe {
            rust_shim_message_body_mut(self).as_mut()
        }
    }
}

#[repr(C)]
struct BodySize {
    known: bool,
    size: u64,
}

impl Body {
    pub fn size(&self) -> Option<u64> {
        unsafe {
            let size = rust_shim_body_size(self);
            if size.known {
                Some(size.size)
            } else {
                None
            }
        }
    }
}

extern "C" {
    fn rust_shim_header_has_any(header: *const Header, name: *const Name) -> bool;
    fn rust_shim_header_value(header: *const Header, name: *const Name) -> RustArea;
    fn rust_shim_header_add(header: *mut Header, name: *const Name, value: *const RustArea);
    fn rust_shim_header_remove_any(header: *mut Header, name: *const Name);
    fn rust_shim_header_image(header: *const Header) -> RustArea;
    fn rust_shim_header_parse(header: *mut Header, buf: *const RustArea);
    fn rust_shim_header_visit_each(header: *const Header, cb: ::ecap::VisitorCallback, extra: *const c_void);
}

impl Header {
    pub fn has_any(&self, name: &Name) -> bool {
        unsafe {
            rust_shim_header_has_any(self, name)
        }
    }

    pub fn value(&self, name: &Name) -> RustArea {
        unsafe {
            rust_shim_header_value(self, name)
        }
    }

    pub fn add(&mut self, name: &Name, value: &RustArea) {
        unsafe {
            rust_shim_header_add(self, name, value)
        }
    }

    pub fn remove_any(&mut self, name: &Name) {
        unsafe {
            rust_shim_header_remove_any(self, name)
        }
    }

    pub fn image(&self) -> RustArea {
        unsafe {
            rust_shim_header_image(self)
        }
    }

    pub fn parse(&mut self, buf: &RustArea) {
        unsafe {
            rust_shim_header_parse(self, buf)
        }
    }

    pub fn visit_each(&self, callback: fn(&Name, &PascalStr)) {
        unsafe {
            rust_shim_header_visit_each(
                self, ::ecap::visitor_callback, callback as *const c_void);
        }
    }
}
