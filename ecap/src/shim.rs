use std::mem;
use std::fmt::{self, Write};
use std::cell::RefCell;
use libc::{c_char, c_void};
use ecap;
use Service;
use Minimal;
use std::ffi::CStr;
use ffi;

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

type ServicePtr = *mut *mut c_void;

unsafe fn to_service<'a>(service: &'a ServicePtr) -> &'a dyn Service {
    assert!(!service.is_null());
    let service: *mut *mut dyn Service = mem::transmute(*service);
    let service = *(service as *mut *mut Service);
    &*service
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_start(service: ServicePtr) {
    to_service(&service).start();
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_stop(service: ServicePtr) {
    to_service(&service).stop();
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_retire(service: ServicePtr) {
    to_service(&service).retire();
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_describe(service: ServicePtr, stream: *mut Ostream) {
    let s = to_service(&service);
    let desc = s.describe();
    write!(&mut *stream, "{}", desc).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_uri(service: ServicePtr) -> ffi::CVec {
    let s = to_service(&service);
    ffi::CVec::from(s.uri())
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_tag(service: ServicePtr) -> ffi::CVec {
    let s = to_service(&service);
    ffi::CVec::from(s.tag())
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_configure(service: ServicePtr, options: *const ecap::Options) {
    assert!(!options.is_null());
    to_service(&service).configure(&*options)
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_reconfigure(service: ServicePtr, options: *const ecap::Options) {
    assert!(!options.is_null());
    to_service(&service).reconfigure(&*options)
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_wants_url(service: ServicePtr, url: *const c_char) -> bool {
    assert!(!url.is_null());
    to_service(&service).wants_url(CStr::from_ptr(url))
}

#[no_mangle]
pub unsafe extern fn rust_service_create() -> ServicePtr {
    // FIXME: This needs to somehow be given the ctor for the service we want to create
    let service = Minimal {
        victim: RefCell::new(None),
        replacement: RefCell::new(None),
    };

    let service: Box<dyn Service> = Box::new(service);
    let ptr = Box::into_raw(service);
    let service_ptr: Box<*mut dyn Service> = Box::new(ptr);
    Box::into_raw(service_ptr) as *mut *mut c_void
}

#[no_mangle]
pub unsafe extern fn rust_service_free(service: ServicePtr) {
    assert!(!service.is_null());
    let service: Box<dyn Service> = Box::from_raw(*(service as *mut *mut Service));
    mem::drop(service);
}
