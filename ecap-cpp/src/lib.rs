extern crate ecap;
extern crate ecap_common_link;
extern crate ecap_sys as ffi;
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
            pub fn as_ptr<'a>(&self) -> *const $cname {
                self as *const _ as *mut $cname
            }

            #[inline]
            pub fn as_ptr_mut<'a>(&mut self) -> *mut $cname {
                self as *mut _ as *mut $cname
            }
        }
    };
}

pub mod adapter;
pub mod common;
pub mod host;

use ecap::adapter::Service;
use libc::c_void;

pub fn register_service<T: Service>(service: T) {
    unsafe {
        let service: Box<dyn Service> = Box::new(service);
        let ptr = Box::into_raw(service);
        let service_ptr: Box<*mut dyn Service> = Box::new(ptr);
        let ptr = Box::into_raw(service_ptr) as *mut *mut c_void;
        ffi::rust_shim_register_service(ptr);
    }
}

use ecap::Translator;

struct CppTranslator;

impl Translator for CppTranslator {
    fn register_service(&self, service: Box<dyn Service + Send + Sync>) {
        let ptr = Box::into_raw(service);
        let thin_ptr = Box::into_raw(Box::new(ptr));
        unsafe {
            ffi::rust_shim_register_service(thin_ptr as *mut *mut c_void);
        }
    }
}

extern "C" fn on_load() {
    ecap_common_link::register_translator(CppTranslator);
}

#[link_section = ".ctors"]
pub static __ON_LOAD_PTR: extern "C" fn() = on_load;
