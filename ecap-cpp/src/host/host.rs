use ecap::common::log::LogVerbosity;
use ecap::host::Host;

use common::log::DebugStream;
use common::message::CppMessage;
use host::transaction::{CppTransaction, CppTransactionRef};

pub struct CppHost;

impl Host for CppHost {
    type Message = CppMessage;
    type MessageRef = CppMessage;
    type DebugStream = DebugStream;
    type Transaction = CppTransaction;
    type TransactionRef = CppTransactionRef;

    fn uri(&self) -> String {
        unimplemented!()
    }

    fn describe(&self) -> String {
        unimplemented!()
    }

    fn open_debug(&mut self, verbosity: LogVerbosity) -> Self::DebugStream {
        unimplemented!()
    }

    fn close_debug(&mut self, stream: Self::DebugStream) {
        unimplemented!()
    }

    fn new_request(&self) -> Self::Message {
        unimplemented!()
    }
    fn new_response(&self) -> Self::Message {
        unimplemented!()
    }
}
