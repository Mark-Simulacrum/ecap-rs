extern crate ecap;
extern crate ecap_common_link;

extern "C" fn on_load() {
    println!("loading minimal adapter crate");
    ecap_common_link::register_service(MinimalService(10));
}

#[link_section = ".ctors"]
pub static _ON_LOAD_PTR: extern "C" fn() = on_load;

use std::ffi::CStr;
use std::time::Duration;

#[derive(Debug)]
pub struct MinimalService(u32);

use ecap::host;
use ecap::common::{Area, Options};
use ecap::adapter::{Service, Transaction};

impl Service for MinimalService {
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

    fn suspend(&self, _duration: &mut Duration) {
        panic!("not an async service");
    }

    fn resume(&self) {
        panic!("not an async service");
    }

    fn retire(&self) {
        println!("retiring minimal service");
    }

    fn describe(&self) -> String {
        format!(
            "A minimal adapter from {} v{}: {:?}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            self
        )
    }

    fn wants_url(&self, _url: &CStr) -> bool {
        true
    }

    fn make_transaction<'a>(&mut self, transaction: &'a mut dyn host::Transaction) -> ecap::AllocatedTransaction<'a> {
        ecap::AllocatedTransaction::new(MinimalTransaction { hostx: transaction })
    }
}

pub struct MinimalTransaction<'a> {
    hostx: &'a mut dyn host::Transaction,
}

impl<'a> Transaction for MinimalTransaction<'a> {
    fn start(&mut self) {
        self.hostx.use_virgin();
    }

    fn stop(&mut self) {}
    fn resume(&mut self) {}
    fn adapted_body_discard(&mut self) {}
    fn adapted_body_make(&mut self) {}
    fn adapted_body_make_more(&mut self) {}
    fn adapted_body_stop_making(&mut self) {}
    fn adapted_body_pause(&mut self) {}
    fn adapted_body_resume(&mut self) {}
    fn adapted_body_content(&mut self, _offset: usize, _size: usize) -> Area {
        Area::from_bytes(&[])
    }
    fn adapted_body_content_shift(&mut self, _size: usize) {}
    fn virgin_body_content_done(&mut self, _at_end: bool) {}
    fn virgin_body_content_available(&mut self) {}
}
