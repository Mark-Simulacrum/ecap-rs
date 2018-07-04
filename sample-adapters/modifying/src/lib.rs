#![feature(used)]
extern crate ecap;
extern crate ecap_common_link;

use std::ffi::CStr;
use std::mem;
use std::rc::Rc;

use ecap::adapter::{Service, Transaction};
use ecap::common::{header::Header, Area, Message, Name, NamedValueVisitor, Options};
use ecap::host::{self, Transaction as HostTransactionTrait};

#[derive(Debug)]
pub struct ModifyService {
    victim: Option<Rc<Vec<u8>>>,
    replacement: Option<Rc<Vec<u8>>>,
}

impl<H> Service<H> for ModifyService
where
    H: host::Host + ?Sized,
    H::Transaction: 'static,
{
    type Transaction = ModifyTransaction;

    fn uri(&self) -> String {
        format!("ecap://rust/sample/modifying")
    }

    fn configure<T: Options + ?Sized>(&mut self, options: &T) {
        options.visit_each(CfgVisitor(self));
        assert!(self.victim.is_some(), "Must have configured a victim");
        assert!(
            self.replacement.is_some(),
            "Must have configured a replacement"
        );
        if self.replacement
            .as_ref()
            .unwrap()
            .windows(self.victim.as_ref().unwrap().len())
            .any(|w| *w == self.victim.as_ref().unwrap()[..])
        {
            panic!("will replace indefinitely");
        }
    }

    fn reconfigure<T: Options + ?Sized>(&mut self, options: &T) {
        // clear the victim and replacement
        let _ = self.victim.take();
        let _ = self.replacement.take();
        options.visit_each(CfgVisitor(self));
        assert!(self.victim.is_some(), "Must have configured a victim");
        assert!(
            self.replacement.is_some(),
            "Must have configured a replacement"
        );
        if self.replacement
            .as_ref()
            .unwrap()
            .windows(self.victim.as_ref().unwrap().len())
            .any(|w| *w == self.victim.as_ref().unwrap()[..])
        {
            panic!("will replace indefinitely");
        }
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
        ModifyTransaction {
            victim: self.victim.as_ref().unwrap().clone(),
            replacement: self.replacement.as_ref().unwrap().clone(),
            sending: State::Undecided,
            receiving: State::Undecided,
            buffer: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Eq, Clone)]
enum State {
    Undecided,
    On,
    Complete,
    Never,
}

pub struct ModifyTransaction {
    receiving: State,
    sending: State,
    victim: Rc<Vec<u8>>,
    replacement: Rc<Vec<u8>>,
    buffer: Vec<u8>,
}

impl ModifyTransaction {
    fn stop_virgin_body<'a, H: host::Host + ?Sized>(&mut self, host: &'a mut H::TransactionRef) {
        if self.receiving == State::On {
            host.virgin_body_stop_making();
            self.receiving = State::Complete;
        } else {
            // we already got the entire body or refused it earlier
            assert_ne!(self.receiving, State::Undecided);
        }
    }

    fn adapt_content(&self, content: &mut Vec<u8>) {
        // this is oversimplified; production code should worry about arbitrary
        // chunk boundaries, content encodings, service reconfigurations, etc.

        let mut pos = 0;
        loop {
            let r = content[pos..]
                .windows(self.victim.len())
                .enumerate()
                .find(|(_, window)| **window == self.victim[..])
                .map(|(idx, _)| idx);
            if let Some(idx) = r {
                let range = (pos + idx)..(pos + idx + self.victim.len());
                mem::drop(content.splice(range, self.replacement.iter().cloned()));
                pos = pos + idx;
            } else {
                // did not find victim in content
                break;
            }
        }
    }
}

impl<H: host::Host + ?Sized> Transaction<H> for ModifyTransaction {
    fn start<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        if host.virgin().body().is_some() {
            self.receiving = State::On;
            host.virgin_body_make();
        } else {
            self.receiving = State::Never;
        }

        let mut adapted = host.virgin().clone();
        // FIXME: assert!(adapted.is_some()); -- can host return None from clone?
        // XXX: This content-length should be gotten from the host.
        let content_length = Name::new_known("Content-Length".as_bytes());
        adapted.header_mut().remove_any(&content_length);

        let name = Name::new_known("X-Ecap".as_bytes());
        // XXX: use host global and get uri
        let value = Area::from_bytes(b"foo");
        adapted.header_mut().insert(name, value);
        if adapted.body().is_none() {
            self.sending = State::Never;
            host.use_adapted(adapted);
        } else {
            host.use_adapted(adapted);
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
    fn adapted_body_discard<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.sending, State::Undecided);
        self.sending = State::Never;
        // we do not need more vb if the host is not interested in ab
        self.stop_virgin_body::<H>(host);
    }
    fn adapted_body_make<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.sending, State::Undecided);
        assert!(host.virgin().body().is_some());

        assert!(self.receiving == State::On || self.receiving == State::Complete);

        self.sending = State::On;
        if !self.buffer.is_empty() {
            host.adapted_body_content_available();
        }
    }
    fn adapted_body_make_more<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.receiving, State::On);
        host.virgin_body_make_more();
    }
    fn adapted_body_stop_making<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        self.sending = State::Complete;
        self.stop_virgin_body::<H>(host);
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
        offset: usize,
        size: usize,
    ) -> Area
    where
        H::TransactionRef: 'a,
    {
        assert!(self.sending == State::On || self.sending == State::Complete);
        if offset + size > self.buffer.len() {
            Area::from_bytes(&self.buffer[offset..])
        } else {
            Area::from_bytes(&self.buffer[offset..offset + size])
        }
    }
    fn adapted_body_content_shift<'a>(&mut self, _host: &'a mut H::TransactionRef, size: usize)
    where
        H::TransactionRef: 'a,
    {
        self.buffer.drain(0..size);
    }
    fn virgin_body_content_done<'a>(&mut self, host: &'a mut H::TransactionRef, at_end: bool)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.receiving, State::On);
        self.stop_virgin_body::<H>(host);
        if self.sending == State::On {
            host.adapted_body_content_done(at_end);
            self.sending = State::Complete;
        }
    }
    fn virgin_body_content_available<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a,
    {
        assert_eq!(self.receiving, State::On);
        let mut bytes: Vec<u8> = host.virgin_body_content(0, usize::max_value())
            .as_bytes()
            .into();
        host.virgin_body_content_shift(bytes.len());
        self.adapt_content(&mut bytes);
        self.buffer.extend(bytes);

        if self.sending == State::On {
            host.adapted_body_content_available();
        }
    }
}

impl Options for ModifyTransaction {
    fn option(&self, _name: &Name) -> Option<&Area> {
        // no meta-information to provide
        None
    }

    fn visit_each<V: NamedValueVisitor>(&self, _visitor: V) {
        // no meta-information to provide
    }
}

struct CfgVisitor<'a>(&'a mut ModifyService);

impl<'a> NamedValueVisitor for CfgVisitor<'a> {
    fn visit(&mut self, name: &Name, value: &Area) {
        let value = value.as_bytes();
        match name.image() {
            Some(b"victim") => {
                if value.is_empty() {
                    panic!("unsupported empty victim");
                }
                self.0.victim = Some(Rc::new(value.into()));
            }
            Some(b"replacement") => {
                self.0.replacement = Some(Rc::new(value.into()));
            }
            _ if name.host_id().is_some() => {
                // skip host options
                return;
            }
            key => {
                panic!("unsupported configuration parameter: {:?}", key);
            }
        }
    }
}

pub extern "C" fn on_load() {
    ecap_common_link::register_erased_service(ModifyService {
        victim: None,
        replacement: None,
    });
}

#[link_section = ".ctors"]
#[used]
pub static _ON_LOAD_PTR: extern "C" fn() = on_load;
