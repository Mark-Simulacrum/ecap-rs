#![feature(used)]
#![allow(unused)]
extern crate ecap;
extern crate ecap_common_link;
extern crate ecap_sys as ffi;
extern crate erased_ecap;
extern crate libc;

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
use libc::c_void;
use std::any::Any;

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
            ffi::rust_shim_register_service(thin_ptr as *mut *mut c_void);
        }
    }
}

pub extern "C" fn on_load() {
    ecap_common_link::register_erased_translator(CppTranslator);
}

#[link_section = ".ctors"]
#[used]
pub static __ON_LOAD_PTR: extern "C" fn() = on_load;
