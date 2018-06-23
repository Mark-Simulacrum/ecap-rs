extern crate ecap;

use std::ffi::CStr;

use ecap::xaction::Transaction;
use ecap::xaction::shim::HostTransaction;
use ecap::{Service, Options, AllocatedTransaction};

#[derive(Debug)]
struct Minimal(u32);

#[no_mangle]
pub extern "C" fn rust_register_services() {
    ecap::register_service(Minimal(0));
    ecap::register_service(Minimal(1));
}

impl Service for Minimal {
    fn make_transaction(&mut self, host: *mut HostTransaction) -> AllocatedTransaction {
        AllocatedTransaction::new(MinimalXaction {
            host: unsafe { Some(&mut *host) },
        })
    }

    fn uri(&self) -> String {
        format!("ecap://e-cap.org/ecap/services/sample/minimal{}", self.0)
    }

    fn configure(&self, _options: &Options) {
        // no configuration
    }

    fn reconfigure(&self, _options: &Options) {
        // no configuration
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
        format!("A minimal adapter from {} v{}: {:?}",
            env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), self)
    }

    fn wants_url(&self, _url: &CStr) -> bool {
        true
    }
}

pub struct MinimalXaction<'a> {
    pub host: Option<&'a mut HostTransaction>,
}

macro_rules! host {
    ($s:expr) => {
        $s.host.as_mut().unwrap()
    }
}

impl<'a> Transaction for MinimalXaction<'a> {
    fn start(&mut self) {
        println!("starting minimal xaction: will just use virgin");
        host!(self).use_virgin();
        self.host = None;
    }

    fn stop(&mut self) {
        let _ = self.host.take();
        println!("stopping xaction");
    }

    fn resume(&mut self) { }
    fn adapted_body_discard(&mut self) { }
    fn adapted_body_make(&mut self) { }
    fn adapted_body_make_more(&mut self) { }
    fn adapted_body_stop_making(&mut self) { }
    fn adapted_body_pause(&mut self) {}
    fn adapted_body_resume(&mut self) {}
    fn adapted_body_content(&mut self, _offset: usize, _size: usize) -> ecap::Area {
        ecap::Area::new()
    }
    fn adapted_body_content_shift(&mut self, _offset: usize) { }
    fn virgin_body_content_done(&mut self, _at_end: bool) { }
    fn virgin_body_content_available(&mut self) { }
}

impl<'a> Drop for MinimalXaction<'a> {
    fn drop(&mut self) {
        if let Some(host) = self.host.take() {
            host.adaptation_aborted();
        }
        println!("dropping minimal xaction!");
    }
}
