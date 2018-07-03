use ecap;
use ecap::common::{Area, Delay};

use common::Message;

use host::Host as ErasedHost;

pub trait Transaction<H: ecap::host::Host + ?Sized> {
    fn virgin(&mut self) -> &mut dyn Message;
    fn cause(&mut self) -> &dyn Message;
    fn adapted(&mut self) -> &mut dyn Message;
    fn use_virgin(&mut self);
    fn use_adapted(&mut self, msg: Box<dyn Message>);
    fn block_virgin(&mut self);
    fn adaptation_delayed(&mut self, delay: &Delay);
    fn adaptation_aborted(&mut self);
    fn resume(&mut self);
    fn virgin_body_discard(&mut self);
    fn virgin_body_make(&mut self);
    fn virgin_body_make_more(&mut self);
    fn virgin_body_stop_making(&mut self);
    fn virgin_body_pause(&mut self);
    fn virgin_body_resume(&mut self);
    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area;
    fn virgin_body_content_shift(&mut self, size: usize);
    fn adapted_body_content_done(&mut self, at_end: bool);
    fn adapted_body_content_available(&mut self);
}

impl ecap::host::Transaction<dyn crate::host::Host> for dyn Transaction<dyn crate::host::Host> {
    fn virgin(&mut self) -> &mut dyn Message {
        Self::virgin(self)
    }
    fn cause(&mut self) -> &dyn Message {
        unimplemented!()
    }
    fn adapted(&mut self) -> &mut dyn Message {
        unimplemented!()
    }
    fn use_virgin(&mut self) {
        Self::use_virgin(self)
    }
    fn use_adapted<M: ecap::common::Message<dyn ErasedHost> + 'static>(&mut self, msg: M) {
        Self::use_adapted(self, Box::new(msg))
    }
    fn block_virgin(&mut self) {
        Self::block_virgin(self)
    }
    fn adaptation_delayed(&mut self, delay: &Delay) {
        Self::adaptation_delayed(self, delay)
    }
    fn adaptation_aborted(&mut self) {
        Self::adaptation_aborted(self)
    }
    fn resume(&mut self) {
        Self::resume(self)
    }
    fn virgin_body_discard(&mut self) {
        Self::virgin_body_discard(self)
    }
    fn virgin_body_make(&mut self) {
        Self::virgin_body_make(self)
    }
    fn virgin_body_make_more(&mut self) {
        Self::virgin_body_make_more(self)
    }
    fn virgin_body_stop_making(&mut self) {
        Self::virgin_body_stop_making(self)
    }
    fn virgin_body_pause(&mut self) {
        Self::virgin_body_pause(self)
    }
    fn virgin_body_resume(&mut self) {
        Self::virgin_body_resume(self)
    }
    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
        Self::virgin_body_content(self, offset, size)
    }
    fn virgin_body_content_shift(&mut self, size: usize) {
        Self::virgin_body_content_shift(self, size)
    }
    fn adapted_body_content_done(&mut self, at_end: bool) {
        Self::adapted_body_content_done(self, at_end)
    }
    fn adapted_body_content_available(&mut self) {
        Self::adapted_body_content_available(self)
    }
}
