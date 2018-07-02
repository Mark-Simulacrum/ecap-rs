extern crate ecap;
extern crate ecap_common_link;

use std::ffi::CStr;

use ecap::adapter::{Service, Transaction};
use ecap::common::{Area, Name, NamedValueVisitor, Options};
use ecap::host;

#[derive(Debug)]
pub struct MinimalService;

impl Service for MinimalService {
    fn uri(&self) -> String {
        format!("ecap://rust/sample/minimal")
    }

    fn configure(&mut self, _options: &Options) {
        // no configuration
    }

    fn reconfigure(&mut self, _options: &Options) {
        // no configuration
    }

    fn tag(&self) -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    fn start(&self) {
        // custom code goes here, but none for this service
    }

    fn stop(&self) {
        // custom code goes here, but none for this service
    }

    fn retire(&self) {
        // custom code goes here, but none for this service
    }

    fn describe(&self) -> String {
        format!(
            "A minimal adapter from {} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
    }

    fn wants_url(&self, _url: &CStr) -> bool {
        // minimal adapter is applied to all messages
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

impl<'a> Options for MinimalTransaction<'a> {
    fn option(&self, _name: &Name) -> Option<Area> {
        // no meta-information to provide
        None
    }

    fn visit_each(&self, _visitor: &mut dyn NamedValueVisitor) {
        // no meta-information to provide
    }
}

extern "C" fn on_load() {
    ecap_common_link::register_service(MinimalService);
}

#[link_section = ".ctors"]
pub static _ON_LOAD_PTR: extern "C" fn() = on_load;
