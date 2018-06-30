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

//impl<H: 'static + ecap::host::Host> ecap::host::Transaction<H> for dyn Transaction<H>
//where
//    H: ?Sized,
//    H::Message: 'static,
//    H::MessageRef: Sized + 'static,
//{
//    fn virgin(&mut self) -> &mut H::MessageRef {
//        let dynamic = Self::virgin(self);
//        dynamic.downcast_mut::<H::MessageRef>().unwrap()
//    }
//    fn cause(&mut self) -> &H::MessageRef {
//        let dynamic = Self::cause(self);
//        dynamic.downcast_ref::<H::MessageRef>().unwrap()
//    }
//    fn adapted(&mut self) -> &mut H::MessageRef {
//        let dynamic = Self::adapted(self);
//        dynamic.downcast_mut::<H::MessageRef>().unwrap()
//    }
//    fn use_virgin(&mut self) {
//        Self::use_virgin(self)
//    }
//    fn use_adapted(&mut self, msg: H::Message) {
//        Self::use_adapted(self, Box::new(msg))
//    }
//    fn block_virgin(&mut self) {
//        Self::block_virgin(self)
//    }
//    fn adaptation_delayed(&mut self, delay: &Delay) {
//        Self::adaptation_delayed(self, delay)
//    }
//    fn adaptation_aborted(&mut self) {
//        Self::adaptation_aborted(self)
//    }
//    fn resume(&mut self) {
//        Self::resume(self)
//    }
//    fn virgin_body_discard(&mut self) {
//        Self::virgin_body_discard(self)
//    }
//    fn virgin_body_make(&mut self) {
//        Self::virgin_body_make(self)
//    }
//    fn virgin_body_make_more(&mut self) {
//        Self::virgin_body_make_more(self)
//    }
//    fn virgin_body_stop_making(&mut self) {
//        Self::virgin_body_stop_making(self)
//    }
//    fn virgin_body_pause(&mut self) {
//        Self::virgin_body_pause(self)
//    }
//    fn virgin_body_resume(&mut self) {
//        Self::virgin_body_resume(self)
//    }
//    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
//        Self::virgin_body_content(self, offset, size)
//    }
//    fn virgin_body_content_shift(&mut self, size: usize) {
//        Self::virgin_body_content_shift(self, size)
//    }
//    fn adapted_body_content_done(&mut self, at_end: bool) {
//        Self::adapted_body_content_done(self, at_end)
//    }
//    fn adapted_body_content_available(&mut self) {
//        Self::adapted_body_content_available(self)
//    }
//}

//impl<U, H> Transaction<H> for U
//where
//    U: ecap::host::Transaction<H> + ?Sized,
//    H: ecap::host::Host + ?Sized,
//    H::Message: 'static,
//    H::Message: Sized,
//    H::MessageRef: 'static,
//    //<H::Message as ecap::common::Message>::Body: Sized,
//    //<H::Message as ecap::common::Message>::FirstLine: Sized,
//    //<H::Message as ecap::common::Message>::Header: Sized,
//    //<H::Message as ecap::common::Message>::Trailer: Sized,
//{
//    fn virgin(&mut self) -> &mut dyn Message {
//        U::virgin(self)
//    }
//    fn cause(&mut self) -> &dyn Message {
//        U::cause(self)
//    }
//    fn adapted(&mut self) -> &mut dyn Message {
//        U::adapted(self)
//    }
//    fn use_virgin(&mut self) {
//        U::use_virgin(self)
//    }
//    fn use_adapted(&mut self, msg: Box<dyn Message>) {
//        match msg.downcast::<H::Message>() {
//            Ok(msg) => U::use_adapted(self, *msg),
//            Err(_) => panic!("only one host's messages should be used at a time"),
//        }
//    }
//    fn block_virgin(&mut self) {
//        U::block_virgin(self)
//    }
//    fn adaptation_delayed(&mut self, delay: &Delay) {
//        U::adaptation_delayed(self, delay)
//    }
//    fn adaptation_aborted(&mut self) {
//        U::adaptation_aborted(self)
//    }
//    fn resume(&mut self) {
//        U::resume(self)
//    }
//    fn virgin_body_discard(&mut self) {
//        U::virgin_body_discard(self)
//    }
//    fn virgin_body_make(&mut self) {
//        U::virgin_body_make(self)
//    }
//    fn virgin_body_make_more(&mut self) {
//        U::virgin_body_make_more(self)
//    }
//    fn virgin_body_stop_making(&mut self) {
//        U::virgin_body_stop_making(self)
//    }
//    fn virgin_body_pause(&mut self) {
//        U::virgin_body_pause(self)
//    }
//    fn virgin_body_resume(&mut self) {
//        U::virgin_body_resume(self)
//    }
//    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
//        U::virgin_body_content(self, offset, size)
//    }
//    fn virgin_body_content_shift(&mut self, size: usize) {
//        U::virgin_body_content_shift(self, size)
//    }
//    fn adapted_body_content_done(&mut self, at_end: bool) {
//        U::adapted_body_content_done(self, at_end)
//    }
//    fn adapted_body_content_available(&mut self) {
//        U::adapted_body_content_available(self)
//    }
//}

// xxx
// xxx
// xxx
// xxx
// xxx
// xxx
// xxx
// xxx

//impl<H: 'static + ecap::host::Host> ecap::host::Transaction<dyn crate::host::Host>
//    for dyn Transaction<H>
//{
//    fn virgin(&mut self) -> &mut dyn Message {
//        unimplemented!()
//    }
//    fn cause(&mut self) -> &dyn Message {
//        unimplemented!()
//    }
//    fn adapted(&mut self) -> &mut dyn Message {
//        unimplemented!()
//    }
//    fn use_virgin(&mut self) {
//        Self::use_virgin(self)
//    }
//    fn use_adapted(&mut self, _msg: Box<dyn Message>) {
//        unimplemented!()
//    }
//    fn block_virgin(&mut self) {
//        Self::block_virgin(self)
//    }
//    fn adaptation_delayed(&mut self, delay: &Delay) {
//        Self::adaptation_delayed(self, delay)
//    }
//    fn adaptation_aborted(&mut self) {
//        Self::adaptation_aborted(self)
//    }
//    fn resume(&mut self) {
//        Self::resume(self)
//    }
//    fn virgin_body_discard(&mut self) {
//        Self::virgin_body_discard(self)
//    }
//    fn virgin_body_make(&mut self) {
//        Self::virgin_body_make(self)
//    }
//    fn virgin_body_make_more(&mut self) {
//        Self::virgin_body_make_more(self)
//    }
//    fn virgin_body_stop_making(&mut self) {
//        Self::virgin_body_stop_making(self)
//    }
//    fn virgin_body_pause(&mut self) {
//        Self::virgin_body_pause(self)
//    }
//    fn virgin_body_resume(&mut self) {
//        Self::virgin_body_resume(self)
//    }
//    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
//        Self::virgin_body_content(self, offset, size)
//    }
//    fn virgin_body_content_shift(&mut self, size: usize) {
//        Self::virgin_body_content_shift(self, size)
//    }
//    fn adapted_body_content_done(&mut self, at_end: bool) {
//        Self::adapted_body_content_done(self, at_end)
//    }
//    fn adapted_body_content_available(&mut self) {
//        Self::adapted_body_content_available(self)
//    }
//}
//
//impl<H: 'static + ecap::host::Host> ecap::host::Transaction<H> for dyn Transaction<H>
//where
//    H: ?Sized,
//    H::Message: 'static,
//    H::MessageRef: Sized + 'static,
//{
//    fn virgin(&mut self) -> &mut H::MessageRef {
//        let dynamic = Self::virgin(self);
//        dynamic.downcast_mut::<H::MessageRef>().unwrap()
//    }
//    fn cause(&mut self) -> &H::MessageRef {
//        let dynamic = Self::cause(self);
//        dynamic.downcast_ref::<H::MessageRef>().unwrap()
//    }
//    fn adapted(&mut self) -> &mut H::MessageRef {
//        let dynamic = Self::adapted(self);
//        dynamic.downcast_mut::<H::MessageRef>().unwrap()
//    }
//    fn use_virgin(&mut self) {
//        Self::use_virgin(self)
//    }
//    fn use_adapted(&mut self, msg: H::Message) {
//        Self::use_adapted(self, Box::new(msg))
//    }
//    fn block_virgin(&mut self) {
//        Self::block_virgin(self)
//    }
//    fn adaptation_delayed(&mut self, delay: &Delay) {
//        Self::adaptation_delayed(self, delay)
//    }
//    fn adaptation_aborted(&mut self) {
//        Self::adaptation_aborted(self)
//    }
//    fn resume(&mut self) {
//        Self::resume(self)
//    }
//    fn virgin_body_discard(&mut self) {
//        Self::virgin_body_discard(self)
//    }
//    fn virgin_body_make(&mut self) {
//        Self::virgin_body_make(self)
//    }
//    fn virgin_body_make_more(&mut self) {
//        Self::virgin_body_make_more(self)
//    }
//    fn virgin_body_stop_making(&mut self) {
//        Self::virgin_body_stop_making(self)
//    }
//    fn virgin_body_pause(&mut self) {
//        Self::virgin_body_pause(self)
//    }
//    fn virgin_body_resume(&mut self) {
//        Self::virgin_body_resume(self)
//    }
//    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
//        Self::virgin_body_content(self, offset, size)
//    }
//    fn virgin_body_content_shift(&mut self, size: usize) {
//        Self::virgin_body_content_shift(self, size)
//    }
//    fn adapted_body_content_done(&mut self, at_end: bool) {
//        Self::adapted_body_content_done(self, at_end)
//    }
//    fn adapted_body_content_available(&mut self) {
//        Self::adapted_body_content_available(self)
//    }
//}
