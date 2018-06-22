#![feature(optin_builtin_traits, extern_types)]

extern crate libc;
extern crate ecap_sys as ffi;

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
    }
}

use std::mem;
use std::fmt::Write;
use std::cell::RefCell;
use std::ffi::CStr;

pub mod log;
pub mod ecap;
pub mod shim;
pub mod xaction;
pub mod message;

pub trait Service {
    fn uri(&self) -> String;
    fn tag(&self) -> String;
    fn describe(&self) -> String;
    fn configure(&self, options: &ecap::Options);
    fn reconfigure(&self, options: &ecap::Options);
    fn start(&self);
    fn stop(&self);
    fn retire(&self);

    fn wants_url(&self, url: &CStr) -> bool;
}

#[derive(Debug)]
struct Minimal {
    victim: RefCell<Option<ecap::Area>>,
    replacement: RefCell<Option<ecap::Area>>,
}

impl Service for Minimal {
    fn uri(&self) -> String {
        format!("ecap://e-cap.org/ecap/services/sample/minimal")
    }

    fn configure(&self, options: &ecap::Options) {
        let victim = options.option(b"victim");
        let replacement = options.option(b"replacement");

        options.visit(|name, value| {
            println!("n={:p}, v={:?}", name, value);
        });

        println!("will replace {:?} with {:?}", victim, replacement);
        *self.victim.borrow_mut() = Some(victim);
        *self.replacement.borrow_mut() = Some(replacement);
    }

    fn reconfigure(&self, _options: &ecap::Options) {
        println!("reconfiguring");
    }

    fn tag(&self) -> String {
        format!("0.0.1")
    }

    fn start(&self) {
        println!("starting minimal service");
    }

    fn stop(&self) {
        println!("stopping minimal service");
    }

    fn retire(&self) {
        println!("retiring minimal service");
    }

    fn describe(&self) -> String {
        println!("host uri: {:?}", String::from_utf8_lossy(&ecap::Host::uri()));

        let mut debug = ecap::DebugStream::new();
        write!(debug, "happiness1").unwrap();
        mem::drop(debug);
        let mut debug = ecap::DebugStream::new();
        write!(debug, "happiness3").unwrap();
        mem::drop(debug);

        format!("A minimal adapter from {} v{}: {:?}",
            env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), self)
    }

    fn wants_url(&self, url: &CStr) -> bool {
        println!("url: {:?}", url);
        true
    }
}
