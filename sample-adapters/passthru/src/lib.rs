#![feature(used)]

extern crate ecap;
extern crate ecap_common_link;

use std::ffi::CStr;

use ecap::adapter::{Service, Transaction};
use ecap::common::{Area, Message, Name, NamedValueVisitor, Options};
use ecap::host::{self, Transaction as HostTransactionTrait};

#[derive(Debug)]
pub struct PassthruService;

impl<H> Service<H> for PassthruService
where
    H: host::Host + ?Sized,
    H::Transaction: 'static,
{
    type Transaction = PassthruTransaction;

    fn uri(&self) -> String {
        format!("ecap://rust/sample/passthru")
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
        PassthruTransaction {
            receiving: State::Undecided,
            sending: State::Undecided,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Undecided,
    On,
    Complete,
    Never,
}

pub struct PassthruTransaction {
    receiving: State,
    sending: State,
}

impl<H: host::Host + ?Sized> Transaction<H> for PassthruTransaction {
    fn start<'a>(&mut self, hostx: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        if hostx.virgin().body().is_some() {
            self.receiving = State::On;
            hostx.virgin_body_make();
        } else {
            self.receiving = State::Never;
        }

        let adapted = hostx.virgin().clone();
        if adapted.body().is_none() {
            self.sending = State::Never;
            hostx.use_adapted(adapted);
        } else {
            hostx.use_adapted(adapted);
        }
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
        assert_eq!(self.sending, State::Undecided);
        self.sending = State::Never;
    }
    fn adapted_body_make<'a>(&mut self, hostx: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.sending, State::Undecided);
        assert!(hostx.virgin().body().is_some());
        assert!(self.receiving == State::On || self.receiving == State::Complete);

        self.sending = State::On;
        hostx.adapted_body_content_available();
    }
    fn adapted_body_make_more<'a>(&mut self, hostx: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.receiving, State::On);
        hostx.virgin_body_make_more();
    }
    fn adapted_body_stop_making<'a>(&mut self, hostx: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.sending, State::Complete);
        hostx.virgin_body_stop_making();
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
        host: &'a mut H::TransactionRef,
        offset: usize,
        size: usize,
    ) -> Area
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.sending, State::On);
        host.virgin_body_content(offset, size)
    }
    fn adapted_body_content_shift<'a>(&mut self, host: &'a mut H::TransactionRef, size: usize)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.sending, State::On);
        host.virgin_body_content_shift(size);
    }
    fn virgin_body_content_done<'a>(&mut self, host: &'a mut H::TransactionRef, at_end: bool)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.receiving, State::On);
        host.virgin_body_stop_making();
        self.receiving = State::Complete;
        host.adapted_body_content_done(at_end);
    }
    fn virgin_body_content_available<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        host.adapted_body_content_available()
    }
}

impl Options for PassthruTransaction {
    fn option(&self, _name: &Name) -> Option<Area> {
        // no meta-information to provide
        None
    }

    fn visit_each<V: NamedValueVisitor>(&self, _visitor: V) {
        // no meta-information to provide
    }
}

extern "C" fn on_load() {
    ecap_common_link::register_erased_service(PassthruService);
}

#[link_section = ".ctors"]
#[used]
pub static ON_LOAD_PTR: extern "C" fn() = on_load;
