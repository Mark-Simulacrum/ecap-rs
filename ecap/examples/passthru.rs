extern crate ecap;

use std::cell::RefCell;
use std::ffi::CStr;

use ecap::xaction::shim::HostTransaction;
use ecap::xaction::Transaction;
use ecap::{AllocatedTransaction, Area, Options, Service};

#[derive(Debug)]
struct Minimal {
    victim: RefCell<Option<Area>>,
    replacement: RefCell<Option<Area>>,
}

#[no_mangle]
pub extern "C" fn rust_register_services() {
    ecap::register_service(Minimal {
        victim: RefCell::new(None),
        replacement: RefCell::new(None),
    });
}

impl Service for Minimal {
    fn make_transaction(&mut self, host: *mut HostTransaction) -> AllocatedTransaction {
        AllocatedTransaction::new(MinimalXaction {
            host: unsafe { Some(&mut *host) },
            sending: State::Undecided,
            receiving: State::Undecided,
        })
    }

    fn uri(&self) -> String {
        format!("ecap://e-cap.org/ecap/services/sample/passthru")
    }

    fn configure(&self, _options: &Options) {
        // no options
    }

    fn reconfigure(&self, _options: &Options) {
        // no options
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
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Undecided,
    On,
    Complete,
    Never,
}

pub struct MinimalXaction<'a> {
    pub host: Option<&'a mut HostTransaction>,
    pub receiving: State,
    pub sending: State,
}

macro_rules! host {
    ($s:expr) => {
        $s.host.as_mut().unwrap()
    };
}

impl<'a> Transaction for MinimalXaction<'a> {
    fn start(&mut self) {
        if host!(self).virgin().body().is_some() {
            self.receiving = State::On;
            host!(self).virgin_body_make();
        } else {
            self.receiving = State::Never;
        }

        let adapted = host!(self).virgin().clone();
        if adapted.body().is_none() {
            self.sending = State::Never; // nothing to send
            host!(self).use_adapted(&adapted);
            self.host = None;
        } else {
            host!(self).use_adapted(&adapted);
        }
    }

    fn stop(&mut self) {
        let _ = self.host.take();
    }

    fn resume(&mut self) {}

    fn adapted_body_discard(&mut self) {
        assert_eq!(self.sending, State::Undecided);
        self.sending = State::Never;
    }

    fn adapted_body_make(&mut self) {
        assert_eq!(self.sending, State::Undecided);
        assert!(host!(self).virgin().body().is_some());
        assert!(self.receiving == State::On || self.receiving == State::Complete);

        self.sending = State::On;
        host!(self).note_adapted_body_content_available();
    }

    fn adapted_body_make_more(&mut self) {
        assert_eq!(self.receiving, State::On);
        host!(self).virgin_body_make_more();
    }

    fn adapted_body_stop_making(&mut self) {
        self.sending = State::Complete;
    }

    fn adapted_body_pause(&mut self) {}
    fn adapted_body_resume(&mut self) {}

    fn adapted_body_content(&mut self, offset: usize, size: usize) -> ecap::Area {
        assert_eq!(self.sending, State::On);
        host!(self).virgin_body_content(offset, size)
    }

    fn adapted_body_content_shift(&mut self, offset: usize) {
        assert_eq!(self.sending, State::On);
        host!(self).virgin_body_content_shift(offset);
    }

    fn virgin_body_content_done(&mut self, at_end: bool) {
        assert_eq!(self.receiving, State::On);
        host!(self).virgin_body_stop_making();
        self.receiving = State::Complete;
        host!(self).note_adapted_body_content_done(at_end);
    }

    fn virgin_body_content_available(&mut self) {
        assert_eq!(self.receiving, State::On);
        if self.sending == State::On {
            host!(self).note_adapted_body_content_available();
        }
    }
}

impl<'a> Drop for MinimalXaction<'a> {
    fn drop(&mut self) {
        if let Some(host) = self.host.take() {
            host.adaptation_aborted();
        }
    }
}
