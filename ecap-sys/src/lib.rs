#![feature(unwind_attributes, extern_types)]

extern crate libc;

use std::marker::PhantomData;
use std::{mem, ptr};

use libc::{c_char, c_int, c_void, size_t};

#[repr(C)]
pub struct Panic {
    pub is_exception: bool,
    pub message: CVec,
    pub location: PanicLocation,
}

#[repr(C)]
pub struct PanicLocation {
    pub file: CVec,
    pub line: c_int,
    pub column: c_int,
}

#[repr(C)]
pub struct ExceptionPtr {
    ptr: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct Version {
    pub major: c_int,
    pub minor: c_int,
    pub micro: c_int,
}

extern "C" {
    pub type FirstLine;
    pub type RequestLine;
    pub type StatusLine;
    pub type Message;
    pub type Header;
    pub type Body;
    pub type HostTransaction;
    pub type Options;
    pub type Host;
    pub type Ostream;
}

#[repr(C)]
pub struct LogVerbosity(pub size_t);

#[repr(C)]
#[repr(align(8))]
pub struct SharedPtrMessage([u8; 16], PhantomData<*mut ()>);

#[repr(C)]
pub struct BodySize {
    pub known: bool,
    pub size: u64,
}

#[repr(C)]
pub struct Name {
    pub image: PStr,
    // Unknown = 0,
    // Unidentified = 1,
    // ... and rest are normal
    pub id: c_int,
    pub host_id: c_int,
    // De-implement send/sync for now
    pub phantom: PhantomData<*mut ()>,
}

#[repr(C)]
pub struct Area {
    pub size: size_t,
    pub buf: *mut c_char,
    // Scratch space for the shared_ptr in C++; we assert that 16
    // bytes is sufficient in C++.
    pub details: [u8; 16],
    pub __align: [u64; 0],
}

#[repr(C)]
pub struct PStr {
    pub size: size_t,
    pub buf: *const c_char,
}

#[repr(C)]
pub struct CVec {
    pub size: size_t,
    pub buf: *const c_char,
    pub capacity: size_t,
}

impl CVec {
    pub fn to_rust(self) -> Vec<u8> {
        let ret = unsafe { Vec::from_raw_parts(self.buf as *mut u8, self.size, self.capacity) };
        mem::forget(self);
        ret
    }
}

impl<T: Into<Vec<u8>>> From<T> for CVec {
    fn from(v: T) -> CVec {
        let data = v.into();
        let ret = CVec {
            size: data.len(),
            buf: data.as_ptr() as *mut c_char,
            capacity: data.capacity(),
        };
        mem::forget(data);
        ret
    }
}

impl Drop for CVec {
    fn drop(&mut self) {
        unsafe {
            let _ = Vec::from_raw_parts(self.buf as *mut u8, self.size, self.capacity);
        }
    }
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_new_string(buf: *const c_char, len: size_t) -> CVec {
    let mut data = Vec::with_capacity(len);
    ptr::copy_nonoverlapping(buf as *const u8, data.as_mut_ptr(), len);
    let v = CVec {
        size: data.len(),
        buf: data.as_ptr() as *mut c_char,
        capacity: data.capacity(),
    };
    mem::forget(data);
    v
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_free_string(s: CVec) {
    mem::drop(s);
}

pub type VisitorCallback = extern "C" fn(Name, Area, *mut c_void);

#[unwind(aborts)]
extern "C" {
    pub fn rust_shim_first_line_version(line: *const FirstLine, out: *mut Version) -> bool;
    pub fn rust_shim_first_line_set_version(line: *mut FirstLine, version: *const Version) -> bool;
    pub fn rust_shim_first_line_protocol(line: *const FirstLine, out: *mut Name) -> bool;
    pub fn rust_shim_first_line_set_protocol(line: *mut FirstLine, protocol: *const Name) -> bool;

    pub fn rust_shim_message_first_line(msg: *const Message, out: *mut *const FirstLine) -> bool;
    pub fn rust_shim_message_first_line_mut(msg: *mut Message, out: *mut *mut FirstLine) -> bool;
    pub fn rust_shim_message_header(msg: *const Message, out: *mut *const Header) -> bool;
    pub fn rust_shim_message_header_mut(msg: *mut Message, out: *mut *mut Header) -> bool;
    pub fn rust_shim_message_add_body(msg: *mut Message) -> bool;
    pub fn rust_shim_message_add_trailer(msg: *mut Message) -> bool;
    pub fn rust_shim_message_trailer(msg: *const Message, out: *mut *const Header) -> bool;
    pub fn rust_shim_message_trailer_mut(msg: *mut Message, out: *mut *mut Header) -> bool;
    pub fn rust_shim_message_body(msg: *const Message, out: *mut *const Body) -> bool;
    pub fn rust_shim_message_body_mut(msg: *mut Message, out: *mut *mut Body) -> bool;
    pub fn rust_shim_message_clone(msg: *const Message, out: *mut SharedPtrMessage) -> bool;

    pub fn rust_shim_shared_ptr_message_ref(msg: *const SharedPtrMessage) -> *const Message;
    pub fn rust_shim_shared_ptr_message_ref_mut(msg: *mut SharedPtrMessage) -> *mut Message;
    pub fn rust_shim_shared_ptr_message_free(msg: *mut SharedPtrMessage);
    pub fn rust_shim_body_size(line: *const Body, out: *mut BodySize) -> bool;

    pub fn rust_shim_header_has_any(
        header: *const Header,
        name: *const Name,
        out: *mut bool,
    ) -> bool;
    pub fn rust_shim_header_value(header: *const Header, name: *const Name, out: *mut Area)
        -> bool;
    pub fn rust_shim_header_add(header: *mut Header, name: *const Name, value: *const Area)
        -> bool;
    pub fn rust_shim_header_remove_any(header: *mut Header, name: *const Name) -> bool;
    pub fn rust_shim_header_image(header: *const Header, out: *mut Area) -> bool;
    pub fn rust_shim_header_parse(header: *mut Header, buf: *const Area) -> bool;
    pub fn rust_shim_header_visit_each(
        header: *const Header,
        cb: VisitorCallback,
        extra: *const c_void,
    ) -> bool;

    pub fn rust_shim_host_xaction_virgin(
        xaction: *mut HostTransaction,
        out: *mut *mut Message,
    ) -> bool;
    pub fn rust_shim_host_xaction_cause(
        xaction: *mut HostTransaction,
        out: *mut *const Message,
    ) -> bool;
    pub fn rust_shim_host_xaction_adapted(
        xaction: *mut HostTransaction,
        out: *mut *mut Message,
    ) -> bool;
    pub fn rust_shim_host_xaction_use_virgin(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_use_adapted(
        xaction: *mut HostTransaction,
        msg: *const SharedPtrMessage,
    ) -> bool;
    pub fn rust_shim_host_xaction_block_virgin(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_adaptation_delayed(
        xaction: *mut HostTransaction,
        delay_state: *const c_char,
        delay_state_len: size_t,
        progress: f64,
    ) -> bool;
    pub fn rust_shim_host_xaction_adaptation_aborted(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_resume(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_vb_discard(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_vb_make(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_vb_stop_making(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_vb_make_more(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_vb_pause(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_vb_resume(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_note_ab_content_available(xaction: *mut HostTransaction) -> bool;
    pub fn rust_shim_host_xaction_note_ab_content_done(
        xaction: *mut HostTransaction,
        end: bool,
    ) -> bool;
    pub fn rust_shim_host_xaction_vb_content(
        xaction: *mut HostTransaction,
        offset: size_t,
        size: size_t,
        out: *mut Area,
    ) -> bool;
    pub fn rust_shim_host_xaction_vb_content_shift(
        xaction: *mut HostTransaction,
        size: size_t,
    ) -> bool;

    pub fn rust_area_new_slice(buf: *const c_char, len: size_t, out: *mut Area) -> bool;
    pub fn rust_area_free(area: *mut Area);

    pub fn options_option(options: *const Options, name: *const Name, out: *mut Area) -> bool;
    pub fn options_visit(options: *const Options, cb: VisitorCallback, extra: *mut c_void) -> bool;

    pub fn rust_host(out: *mut *const Host) -> bool;
    pub fn rust_shim_host_uri(host: *const Host, out: *mut CVec) -> bool;
    pub fn rust_shim_host_describe(host: *const Host, out: *mut CVec) -> bool;
    pub fn rust_shim_host_open_debug(
        host: *const Host,
        verbosity: LogVerbosity,
        out: *mut *mut Ostream,
    ) -> bool;
    pub fn rust_shim_host_close_debug(host: *const Host, stream: *mut Ostream) -> bool;
    pub fn rust_shim_host_new_request(host: *const Host, out: *mut SharedPtrMessage) -> bool;
    pub fn rust_shim_host_new_response(host: *const Host, out: *mut SharedPtrMessage) -> bool;
    pub fn rust_shim_ostream_write(stream: *mut Ostream, buf: *const c_char, len: size_t) -> bool;

    pub fn rust_shim_register_service(service: *mut *mut c_void, out: *mut bool) -> bool;
}
