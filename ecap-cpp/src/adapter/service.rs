use ffi;
use libc::{c_char, c_void, timeval};
use std::ffi::CStr;
use std::panic;
use std::time::Duration;
use std::{fmt::Write, mem};

use common::log::Ostream;
use common::options::CppOptions;
use ffi_unwind;

use erased_ecap::adapter::Service as ErasedService;
use erased_ecap::host::Host;

pub type ServicePtr = *mut *mut c_void;

unsafe fn to_service<'a>(service: &'a ServicePtr) -> &'a dyn ErasedService<dyn Host> {
    assert!(!service.is_null());
    let service: *mut *mut dyn ErasedService<dyn Host> = mem::transmute(*service);
    let service = *service;
    &*service
}

pub unsafe fn to_service_mut<'a>(
    service: &'a mut ServicePtr,
) -> &'a mut dyn ErasedService<dyn Host> {
    assert!(!service.is_null());
    let service: *mut *mut dyn ErasedService<dyn Host> = mem::transmute(*service);
    let service = *service;
    &mut *service
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_start(service: ServicePtr) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| to_service(&service).start()),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_stop(service: ServicePtr) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| to_service(&service).stop()),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_retire(service: ServicePtr) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| to_service(&service).retire()),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_is_async(service: ServicePtr, out: *mut bool) -> bool {
    ffi_unwind(
        out,
        panic::AssertUnwindSafe(|| to_service(&service).is_async()),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_resume(service: ServicePtr) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| to_service(&service).resume()),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_suspend(service: ServicePtr, duration: *mut timeval) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| {
            assert!(!duration.is_null());
            let duration = &mut *duration;
            let mut rduration = Duration::from_secs(duration.tv_sec as u64);
            rduration += Duration::from_micros(duration.tv_usec as u64);
            to_service(&service).suspend(&mut rduration);
            duration.tv_sec = rduration.as_secs() as i64;
            duration.tv_usec = rduration.subsec_micros() as i64;
        }),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_describe(
    service: ServicePtr,
    stream: *mut ffi::Ostream,
) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| {
            let s = to_service(&service);
            let desc = s.describe();
            write!(Ostream::from_ptr_mut(stream), "{}", desc).unwrap();
        }),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_uri(service: ServicePtr, out: *mut ffi::CVec) -> bool {
    ffi_unwind(
        out,
        panic::AssertUnwindSafe(|| {
            let s = to_service(&service);
            ffi::CVec::from(s.uri())
        }),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_tag(service: ServicePtr, out: *mut ffi::CVec) -> bool {
    ffi_unwind(
        out,
        panic::AssertUnwindSafe(|| {
            let s = to_service(&service);
            ffi::CVec::from(s.tag())
        }),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_configure(
    mut service: ServicePtr,
    options: *const ffi::Options,
) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| {
            assert!(!options.is_null());
            to_service_mut(&mut service).configure(CppOptions::from_ptr(options))
        }),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_reconfigure(
    mut service: ServicePtr,
    options: *const ffi::Options,
) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| {
            assert!(!options.is_null());
            to_service_mut(&mut service).reconfigure(CppOptions::from_ptr(options))
        }),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_wants_url(
    service: ServicePtr,
    url: *const c_char,
    out: *mut bool,
) -> bool {
    ffi_unwind(
        out,
        panic::AssertUnwindSafe(|| {
            assert!(!url.is_null());
            to_service(&service).wants_url(CStr::from_ptr(url))
        }),
    )
}

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_service_free(service: ServicePtr) -> bool {
    ffi_unwind(
        &mut (),
        panic::AssertUnwindSafe(|| {
            assert!(!service.is_null());
            let service: Box<dyn ErasedService<dyn Host>> =
                Box::from_raw(*(service as *mut *mut ErasedService<dyn Host>));
            mem::drop(service);
        }),
    )
}
