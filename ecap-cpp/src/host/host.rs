use ecap::common::log::LogVerbosity;
use ecap::host::Host;
use ffi;

use call_ffi_maybe_panic;
use std::panic;

use common::body::CppBody;
use common::log::DebugStream;
use common::message::{CppFirstLine, CppHeader, CppMessage, SharedPtrMessage};
use host::transaction::{CppTransaction, CppTransactionRef};

use std::mem;

foreign_ref!(pub struct CppHost(ffi::Host));

impl CppHost {
    // XXX: Is this really &'static? Can the adapter rely on that?
    pub fn new() -> &'static CppHost {
        unsafe {
            let host = call_ffi_maybe_panic(|out| unsafe { ffi::rust_host(out) });
            assert!(!host.is_null());
            CppHost::from_ptr(host)
        }
    }
}

impl Host for CppHost {
    type Message = CppMessage;
    type MessageRef = CppMessage;
    type DebugStream = DebugStream;
    type Transaction = CppTransaction;
    type TransactionRef = CppTransactionRef;
    type Body = CppBody;
    type Header = CppHeader;
    type FirstLine = CppFirstLine;
    type Trailer = CppHeader;

    fn uri(&self) -> String {
        unsafe {
            let v =
                call_ffi_maybe_panic(|out| unsafe { ffi::rust_shim_host_uri(self.as_ptr(), out) });
            // FIXME: Should we be returning Vec<u8> here?
            String::from_utf8(v.to_rust()).unwrap()
        }
    }

    fn describe(&self) -> String {
        unsafe {
            let v = call_ffi_maybe_panic(|out| unsafe {
                ffi::rust_shim_host_describe(self.as_ptr(), out)
            });
            // FIXME: Should we be returning Vec<u8> here?
            String::from_utf8(v.to_rust()).unwrap()
        }
    }

    fn open_debug(&self, verbosity: LogVerbosity) -> Option<Self::DebugStream> {
        DebugStream::from_host(self, verbosity)
    }

    // Dropping the stream closes it.
    fn close_debug(&self, _stream: Self::DebugStream) {}

    fn new_request(&self) -> SharedPtrMessage {
        let raw = call_ffi_maybe_panic(|out| unsafe {
            ffi::rust_shim_host_new_request(self.as_ptr(), out)
        });
        SharedPtrMessage(raw)
    }
    fn new_response(&self) -> SharedPtrMessage {
        let raw = call_ffi_maybe_panic(|out| unsafe {
            ffi::rust_shim_host_new_response(self.as_ptr(), out)
        });
        SharedPtrMessage(raw)
    }
}
