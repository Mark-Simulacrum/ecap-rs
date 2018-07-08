#![feature(used)]

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
    type Transaction = MinimalTransaction;

    fn uri(&self) -> String {
        format!("ecap://rust/sample/minimal")
    }

    fn configure<T: Options + ?Sized>(&mut self, _options: &T) {
        // no configuration
    }

    fn reconfigure<T: Options + ?Sized>(&mut self, _options: &T) {
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

    fn make_transaction(&mut self, _transaction: &mut H::TransactionRef) -> Self::Transaction {
        MinimalTransaction
        //ecap::AllocatedTransaction::new(MinimalTransaction { hostx: transaction })
    }
}

pub struct MinimalTransaction;

impl<H: host::Host + ?Sized> Transaction<H> for MinimalTransaction {
    fn start<'a>(&mut self, hostx: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        hostx.use_virgin();
    }

    fn stop<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
    fn resume<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
    fn adapted_body_discard<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
    fn adapted_body_make<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
    fn adapted_body_make_more<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
    fn adapted_body_stop_making<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
    fn adapted_body_pause<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
    fn adapted_body_resume<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
    fn adapted_body_content<'a>(
        &mut self,
        _host: &'a mut H::TransactionRef,
        _offset: usize,
        _size: usize,
    ) -> Area
    where
        H::TransactionRef: 'a,
    {
        Area::from_bytes(&[])
    }
    fn adapted_body_content_shift<'a>(&mut self, _host: &'a mut H::TransactionRef, _size: usize)
    where
        H::TransactionRef: 'a,
    {
    }
    fn virgin_body_content_done<'a>(&mut self, _host: &'a mut H::TransactionRef, _at_end: bool)
    where
        H::TransactionRef: 'a,
    {
    }
    fn virgin_body_content_available<'a>(&mut self, _host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
    }
}

impl Options for MinimalTransaction {
    fn option(&self, _name: &Name) -> Option<Area> {
        // no meta-information to provide
        None
    }

    fn visit_each<V: NamedValueVisitor>(&self, _visitor: V) {
        // no meta-information to provide
    }
}

pub extern "C" fn on_load() {
    ecap_common_link::register_erased_service(MinimalService);
}

#[link_section = ".ctors"]
#[used]
pub static ON_LOAD_PTR: extern "C" fn() = on_load;
