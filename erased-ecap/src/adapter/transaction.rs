use ecap;
use ecap::common::{Area, Name, NamedValueVisitor};

use common;

pub trait Transaction: common::Options {
    fn start(&mut self);
    fn stop(&mut self);
    fn resume(&mut self);
    fn adapted_body_discard(&mut self);
    fn adapted_body_make(&mut self);
    fn adapted_body_make_more(&mut self);
    fn adapted_body_stop_making(&mut self);
    fn adapted_body_pause(&mut self);
    fn adapted_body_resume(&mut self);
    fn adapted_body_content(&mut self, offset: usize, size: usize) -> Area;
    fn adapted_body_content_shift(&mut self, size: usize);
    fn virgin_body_content_done(&mut self, at_end: bool);
    fn virgin_body_content_available(&mut self);
}

impl<U> Transaction for U
where
    U: ecap::adapter::Transaction + ?Sized,
{
    fn start(&mut self) {
        U::start(self)
    }
    fn stop(&mut self) {
        U::stop(self)
    }
    fn resume(&mut self) {
        U::resume(self)
    }
    fn adapted_body_discard(&mut self) {
        U::adapted_body_discard(self)
    }
    fn adapted_body_make(&mut self) {
        U::adapted_body_make(self)
    }
    fn adapted_body_make_more(&mut self) {
        U::adapted_body_make_more(self)
    }
    fn adapted_body_stop_making(&mut self) {
        U::adapted_body_stop_making(self)
    }
    fn adapted_body_pause(&mut self) {
        U::adapted_body_pause(self)
    }
    fn adapted_body_resume(&mut self) {
        U::adapted_body_resume(self)
    }
    fn adapted_body_content(&mut self, offset: usize, size: usize) -> Area {
        U::adapted_body_content(self, offset, size)
    }
    fn adapted_body_content_shift(&mut self, size: usize) {
        U::adapted_body_content_shift(self, size)
    }
    fn virgin_body_content_done(&mut self, at_end: bool) {
        U::virgin_body_content_done(self, at_end)
    }
    fn virgin_body_content_available(&mut self) {
        U::virgin_body_content_available(self)
    }
}

impl ecap::adapter::Transaction for dyn Transaction {
    fn start(&mut self) {
        Self::start(self)
    }
    fn stop(&mut self) {
        Self::stop(self)
    }
    fn resume(&mut self) {
        Self::resume(self)
    }
    fn adapted_body_discard(&mut self) {
        Self::adapted_body_discard(self)
    }
    fn adapted_body_make(&mut self) {
        Self::adapted_body_make(self)
    }
    fn adapted_body_make_more(&mut self) {
        Self::adapted_body_make_more(self)
    }
    fn adapted_body_stop_making(&mut self) {
        Self::adapted_body_stop_making(self)
    }
    fn adapted_body_pause(&mut self) {
        Self::adapted_body_pause(self)
    }
    fn adapted_body_resume(&mut self) {
        Self::adapted_body_resume(self)
    }
    fn adapted_body_content(&mut self, offset: usize, size: usize) -> Area {
        Self::adapted_body_content(self, offset, size)
    }
    fn adapted_body_content_shift(&mut self, size: usize) {
        Self::adapted_body_content_shift(self, size)
    }
    fn virgin_body_content_done(&mut self, at_end: bool) {
        Self::virgin_body_content_done(self, at_end)
    }
    fn virgin_body_content_available(&mut self) {
        Self::virgin_body_content_available(self)
    }
}

impl<'a> ecap::common::Options for dyn Transaction + 'a {
    fn option(&self, name: &Name) -> Option<&Area> {
        self.option(name)
    }

    fn visit_each<V: NamedValueVisitor>(&self, mut visitor: V) {
        self.visit_each(&mut visitor)
    }
}
