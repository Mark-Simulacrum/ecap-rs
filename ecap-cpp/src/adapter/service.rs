use ecap::adapter::Service;
use std::{mem, fmt::Write};
use libc::{timeval, c_char, c_void};
use std::time::Duration;
use std::ffi::CStr;
use ffi;

use common::options::CppOptions;
use common::log::Ostream;

pub type ServicePtr = *mut *mut c_void;

unsafe fn to_service<'a>(service: &'a ServicePtr) -> &'a dyn Service {
    assert!(!service.is_null());
    let service: *mut *mut dyn Service = mem::transmute(*service);
    let service = *(service as *mut *mut Service);
    &*service
}

pub unsafe fn to_service_mut<'a>(service: &'a mut ServicePtr) -> &'a mut dyn Service {
    assert!(!service.is_null());
    let service: *mut *mut dyn Service = mem::transmute(*service);
    let service = *(service as *mut *mut Service);
    &mut *service
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
pub unsafe extern "C" fn rust_service_is_async(service: ServicePtr) {
    to_service(&service).is_async();
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_resume(service: ServicePtr) {
    to_service(&service).resume();
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_suspend(service: ServicePtr, duration: *mut timeval) {
    assert!(!duration.is_null());
    let duration = &mut *duration;
    let mut rduration = Duration::from_secs(duration.tv_sec as u64);
    rduration += Duration::from_micros(duration.tv_usec as u64);
    to_service(&service).suspend(&mut rduration);
    duration.tv_sec = rduration.as_secs() as i64;
    duration.tv_usec = rduration.subsec_micros() as i64;
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_describe(service: ServicePtr, stream: *mut ffi::Ostream) {
    let s = to_service(&service);
    let desc = s.describe();
    write!(Ostream::from_ptr_mut(stream), "{}", desc).unwrap();
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
pub unsafe extern "C" fn rust_service_configure(service: ServicePtr, options: *const ffi::Options) {
    assert!(!options.is_null());
    to_service(&service).configure(CppOptions::from_ptr(options))
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_reconfigure(service: ServicePtr, options: *const ffi::Options) {
    assert!(!options.is_null());
    to_service(&service).reconfigure(CppOptions::from_ptr(options))
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_wants_url(service: ServicePtr, url: *const c_char) -> bool {
    assert!(!url.is_null());
    to_service(&service).wants_url(CStr::from_ptr(url))
}

#[no_mangle]
pub unsafe extern "C" fn rust_service_free(service: ServicePtr) {
    assert!(!service.is_null());
    let service: Box<dyn Service> = Box::from_raw(*(service as *mut *mut Service));
    mem::drop(service);
}
