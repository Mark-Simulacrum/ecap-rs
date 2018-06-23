#![feature(crate_visibility_modifier, optin_builtin_traits, extern_types)]

extern crate ecap_sys as ffi;
extern crate libc;

use libc::c_void;

macro_rules! accessor {
    (fn $name:ident() -> &mut $rty:path: $cfunc:path) => {
        pub fn $name(&mut self) -> &mut $rty {
            unsafe {
                <$rty>::from_ptr_mut($cfunc(self.as_ptr_mut()))
            }
        }
    };
    (fn $name:ident() -> &$rty:path: $cfunc:path) => {
        pub fn $name(&self) -> &$rty {
            unsafe {
                <$rty>::from_ptr($cfunc(self.as_ptr()))
            }
        }
    };
    (fn $name:ident(&mut self) -> &$rty:path: $cfunc:path) => {
        pub fn $name(&mut self) -> &$rty {
            unsafe {
                <$rty>::from_ptr($cfunc(self.as_ptr_mut()))
            }
        }
    };
}

macro_rules! foreign_ref {
    (pub struct $name:ident($cname:path)) => {
        pub struct $name($cname);

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

use std::ffi::CStr;

mod area;
pub use area::Area;
mod name;
pub use name::Name;
mod options;
pub use options::Options;
pub mod log;

mod misc;
pub use misc::*;

pub mod message;
pub mod service_shim;
pub mod xaction;

// XXX: Naming
pub struct AllocatedTransaction(Box<dyn xaction::Transaction>);

impl AllocatedTransaction {
    pub fn new<T: xaction::Transaction + 'static>(transaction: T) -> Self {
        AllocatedTransaction(Box::new(transaction))
    }
}

// XXX: The destructor of Service implementors doesn't run today.
pub trait Service {
    fn uri(&self) -> String;
    fn tag(&self) -> String;
    fn describe(&self) -> String;
    fn configure(&self, options: &Options);
    // Not actually called today: probably a bug in Squid.
    fn reconfigure(&self, options: &Options);
    fn start(&self);
    fn stop(&self);
    fn retire(&self);

    fn wants_url(&self, url: &CStr) -> bool;

    fn make_transaction(
        &mut self,
        host: *mut xaction::shim::HostTransaction,
    ) -> AllocatedTransaction;
}

pub fn register_service<T: Service>(service: T) {
    unsafe {
        let service: Box<dyn Service> = Box::new(service);
        let ptr = Box::into_raw(service);
        let service_ptr: Box<*mut dyn Service> = Box::new(ptr);
        let ptr = Box::into_raw(service_ptr) as *mut *mut c_void;
        ffi::rust_shim_register_service(ptr);
    }
}

// Dummy registrar function so that we successfully link with the shim
#[cfg(test)]
#[no_mangle]
pub extern "C" fn rust_register_services() {}
