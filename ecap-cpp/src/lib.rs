#![feature(unwind_attributes, used)]
#![allow(unused)]
extern crate crossbeam;
extern crate ecap;
extern crate ecap_common_link;
extern crate ecap_sys as ffi;
extern crate erased_ecap;
extern crate libc;
#[macro_use]
extern crate lazy_static;

macro_rules! foreign_ref {
    (pub struct $name:ident($cname:path)) => {
        pub struct $name(::std::marker::PhantomData<*mut ()>);

        impl $name {
            #[inline]
            pub unsafe fn from_ptr<'a>(p: *const $cname) -> &'a Self {
                assert!(!p.is_null());
                &*(p as *mut _)
            }

            #[inline]
            pub unsafe fn from_ptr_mut<'a>(p: *mut $cname) -> &'a mut Self {
                assert!(!p.is_null());
                &mut *(p as *mut _)
            }

            #[inline]
            pub unsafe fn from_ptr_mut_opt(p: *mut $cname) -> *mut Self {
                p as *mut _
            }

            #[inline]
            pub fn as_ptr<'a>(&self) -> *const $cname {
                self as *const Self as *const $cname
            }

            #[inline]
            pub fn as_ptr_mut<'a>(&mut self) -> *mut $cname {
                self as *mut Self as *mut $cname
            }
        }
    };
}

pub mod adapter;
pub mod common;
pub mod host;

use ecap::adapter::Service;
use ecap::host::Host;
use libc::{c_int, c_void};
use std::any::Any;
use std::ptr;

use ecap::Translator;
use erased_ecap::adapter::Service as ErasedService;
use erased_ecap::host::Host as ErasedHost;

struct CppTranslator;

impl Translator for CppTranslator {
    fn register_service<H, T>(&self, service: T)
    where
        H: Host + ?Sized,
        T: Service<H> + 'static,
    {
        // We only support this kind of service anyway, so just assert
        // that that's what we got. It's possible that Translator should
        // not be implemented for all services as it is today or some
        // other kind of work should go into this bit.
        {
            let service_any: &Any = &service;
            assert!(service_any.is::<Box<dyn ErasedService<dyn ErasedHost>>>());
        }
        let thin_ptr = Box::into_raw(Box::new(service));
        unsafe {
            assert!(call_ffi_maybe_panic(|raw| unsafe {
                ffi::rust_shim_register_service(thin_ptr as *mut *mut c_void, raw)
            }));
        }
    }
}

use crossbeam::sync::TreiberStack;
lazy_static! {
    static ref PANICS: TreiberStack<PanicPayload> = TreiberStack::new();
}

#[derive(Debug)]
struct PanicLocation {
    file: String,
    line: u32,
    column: u32,
}

#[derive(Debug)]
struct PanicPayload {
    is_exception: bool,
    payload: String,
    location: Option<PanicLocation>,
}

use std::panic::{self, PanicInfo};
fn panic_hook(info: &PanicInfo) {
    let mut is_exception = false;
    let payload = if let Some(s) = info.payload().downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = info.payload().downcast_ref::<&'static str>() {
        String::from(*s)
    } else if let Some(s) = info.payload().downcast_ref::<CppError>() {
        is_exception = true;
        String::from("C++ exception")
    } else {
        String::from("unknown payload")
    };
    let ret = PanicPayload {
        payload: payload,
        is_exception,
        location: info.location().map(|l| PanicLocation {
            file: l.file().to_owned(),
            line: l.line(),
            column: l.column(),
        }),
    };
    PANICS.push(ret);
}

impl PanicPayload {
    fn into_ffi(self) -> ffi::Panic {
        ffi::Panic {
            is_exception: self.is_exception,
            message: self.payload.into(),
            location: self.location
                .map(|l| ffi::PanicLocation {
                    file: l.file.into(),
                    line: l.line as c_int,
                    column: l.column as c_int,
                })
                .unwrap_or(ffi::PanicLocation {
                    file: ffi::CVec::from(vec![]),
                    line: 0,
                    column: 0,
                }),
        }
    }
}

// This is intended to signal in a panic that the error occurred in C++...
struct CppError;

#[no_mangle]
#[unwind(aborts)]
pub unsafe extern "C" fn rust_panic_pop(panic: *mut ffi::Panic) -> bool {
    // This code should be panic-free as they will not be properly handled by it.
    let next = match PANICS.try_pop() {
        Some(n) => n,
        None => return false,
    };

    ptr::write(panic, next.into_ffi());

    true
}

#[no_mangle]
#[unwind(aborts)]
pub extern "C" fn rust_panic_free(panic: ffi::Panic) {
    // We are just dropping vectors here which should be panic-free; this is std
    // code, not user code.
    let _ = panic.message.to_rust();
    let _ = panic.location.file.to_rust();
}

#[must_use]
pub unsafe fn ffi_unwind<F, R>(out: *mut R, f: F) -> bool
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    match panic::catch_unwind(f) {
        Ok(res) => {
            ptr::write(out, res);
            true
        }
        Err(_) => false,
    }
}

use std::mem::{self, ManuallyDrop};

pub fn call_ffi_maybe_panic<F, R>(f: F) -> R
where
    F: FnOnce(*mut R) -> bool,
{
    unsafe {
        let mut raw: ManuallyDrop<R> = ManuallyDrop::new(mem::uninitialized());
        let res = f(&mut *raw);
        if res {
            ManuallyDrop::into_inner(raw)
        } else {
            panic!(::CppError);
        }
    }
}

pub extern "C" fn on_load() {
    panic::set_hook(Box::new(panic_hook));
    ecap_common_link::register_erased_translator(CppTranslator);
}

#[link_section = ".ctors"]
#[used]
pub static __ON_LOAD_PTR: extern "C" fn() = on_load;
