use ecap::common::log::LogVerbosity;
use ecap::host::Host;

use common::body::CppBody;
use common::log::DebugStream;
use common::message::{CppFirstLine, CppHeader, CppMessage, CppTrailer, SharedPtrMessage};
use host::transaction::{CppTransaction, CppTransactionRef};

pub struct CppHost;

impl Host for CppHost {
    type Message = CppMessage;
    type MessageRef = CppMessage;
    type DebugStream = DebugStream;
    type Transaction = CppTransaction;
    type TransactionRef = CppTransactionRef;
    type Body = CppBody;
    type Header = CppHeader;
    type FirstLine = CppFirstLine;
    type Trailer = CppTrailer;

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

    fn new_request(&self) -> SharedPtrMessage {
        unimplemented!()
    }
    fn new_response(&self) -> SharedPtrMessage {
        unimplemented!()
    }
}
