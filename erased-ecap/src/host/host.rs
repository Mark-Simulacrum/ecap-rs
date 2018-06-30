use ecap;
use ecap::common::log::LogVerbosity;
//use std::ffi::CStr;

//use adapter::{ErasedService, Service};
use common::{log::DebugStream, Message};
use host::Transaction;

pub trait Host {
    fn uri(&self) -> String;
    fn describe(&self) -> String;
    //fn note_versioned_service(&mut self, ecap_version: &CStr, service: ErasedService);
    fn open_debug(&mut self, verbosity: LogVerbosity) -> Box<dyn DebugStream>;
    fn close_debug(&mut self, stream: Box<dyn DebugStream>);
    fn new_request(&self) -> Box<dyn Message>;
    fn new_response(&self) -> Box<dyn Message>;
}

impl ecap::host::Host for dyn Host {
    type Message = Box<dyn Message>;
    type DebugStream = Box<dyn DebugStream>;
    type Transaction = Box<dyn Transaction<dyn Host>>;

    fn uri(&self) -> String {
        (&*self).uri()
    }
    fn describe(&self) -> String {
        Self::describe(self)
    }
    //fn note_versioned_service<T: Service<Self>>(
    //    &mut self,
    //    ecap_version: &CStr,
    //    service: T
    //);
    fn open_debug(&mut self, verbosity: LogVerbosity) -> Self::DebugStream {
        (&mut *self).open_debug(verbosity)
    }
    fn close_debug(&mut self, stream: Self::DebugStream) {
        (&mut *self).close_debug(stream)
    }
    fn new_request(&self) -> Self::Message {
        (&*self).new_request()
    }
    fn new_response(&self) -> Self::Message {
        (&*self).new_response()
    }
}

impl<DS, M, H> Host for H
where
    H: ecap::host::Host<Message = M, DebugStream = DS> + 'static + ?Sized,
    M: ecap::common::Message + 'static,
    DS: ecap::common::log::DebugStream + 'static,
{
    fn uri(&self) -> String {
        H::uri(self)
    }
    fn describe(&self) -> String {
        H::describe(self)
    }
    //fn note_versioned_service(&mut self, ecap_version: &CStr, service: ErasedService) {
    //    H::note_versioned_service(self, ecap_version, service.take::<H>())
    //}
    fn open_debug(&mut self, verbosity: LogVerbosity) -> Box<dyn DebugStream> {
        Box::new(H::open_debug(self, verbosity))
    }
    fn close_debug(&mut self, stream: Box<dyn DebugStream>) {
        match stream.downcast::<DS>() {
            Ok(stream) => H::close_debug(self, *stream),
            Err(_) => panic!("streams passed to hosts need to come from the same host"),
        }
    }
    fn new_request(&self) -> Box<dyn Message> {
        Box::new(H::new_request(self))
    }
    fn new_response(&self) -> Box<dyn Message> {
        Box::new(H::new_response(self))
    }
}
