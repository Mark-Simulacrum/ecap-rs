extern crate ecap;
extern crate ecap_common_link;

use std::ffi::CStr;

use ecap::adapter::{Service, Transaction};
use ecap::common::{Area, Name, NamedValueVisitor, Options};
use ecap::host::{self, Transaction as HostTransactionTrait};

#[derive(Debug)]
pub struct MinimalService;

impl<H> Service<H> for MinimalService
where
    H: host::Host + ?Sized,
    H::Transaction: 'static,
{
    type Transaction = MinimalTransaction<'static, H>;

    fn uri(&self) -> String {
        format!("ecap://rust/sample/minimal")
    }

    fn configure<T: Options + ?Sized>(&self, _options: &T) {
        // no configuration
    }

    fn reconfigure<T: Options + ?Sized>(&self, _options: &T) {
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

    fn make_transaction(&mut self, _transaction: &mut H::Transaction) -> Self::Transaction {
        unimplemented!()
        //MinimalTransaction { hostx: transaction }
        //ecap::AllocatedTransaction::new(MinimalTransaction { hostx: transaction })
    }
}

pub struct MinimalTransaction<'a, H: host::Host + ?Sized>
where
    H::Transaction: 'a,
{
    hostx: &'a mut H::Transaction,
}

impl<'a, H: ?Sized + host::Host> Transaction for MinimalTransaction<'a, H> {
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

impl<'a, H: ?Sized + host::Host> Options for MinimalTransaction<'a, H> {
    fn option(&self, _name: &Name) -> Option<&Area> {
        // no meta-information to provide
        None
    }

    fn visit_each<V: NamedValueVisitor>(&self, _visitor: V) {
        // no meta-information to provide
    }
}

extern "C" fn on_load() {
    ecap_common_link::register_erased_service(MinimalService);
}

#[link_section = ".ctors"]
pub static _ON_LOAD_PTR: extern "C" fn() = on_load;
