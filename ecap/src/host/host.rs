use log::{DebugStream, LogVerbosity};
use message::SharedPtrMessage;
use std::ffi::CStr;

/// The primary interface for talking to the host itself.
pub trait Host {
    /// A unique identifer across all vendors.
    fn uri(&self) -> String;

    /// A description of the Host, free-format.
    fn describe(&self) -> String;

    /// Register a versioned service for a given eCAP version.
    ///
    /// The version is not encoded in the Service as it may not be safely readable if the Service
    /// was implemented for a different version of libecap.
    ///
    /// XXX: Investigate and document why this takes a weak_ptr.
    fn note_versioned_service(
        &mut self,
        ecap_version: &CStr,
        service: /*&WeakPtr<Service>*/&(),
    );

    /// Open a logging stream with the given verbosity.
    ///
    /// This may not return a debug stream if the host does not wish to
    /// log at the given verbosity. It does not indicate that will not
    /// change in the future.
    ///
    /// This absence of a DebugStream is hidden inside the `DebugStream`
    /// type for ease of use.
    fn open_debug(&mut self, verbosity: LogVerbosity) -> DebugStream;

    /// Close a debug stream.
    ///
    /// This will line-terminate the debug stream, as well as optionally
    /// prepend a "header" to the stream.
    fn close_debug(&mut self, stream: DebugStream);

    /// Create a fresh request.
    ///
    /// Utilized when copying an existing Message is not appropriate.
    fn new_request(&self) -> SharedPtrMessage;

    /// Create a fresh response.
    ///
    /// Utilized when copying an existing Message is not appropriate.
    fn new_response(&self) -> SharedPtrMessage;
}
