use common::{Area, Delay, Message};
use host::Host;

/// The host side of the eCAP transaction.
///
/// adapter::Transaction implementors use this interface to get virgin messages.
pub trait Transaction<H: ?Sized + Host> {
    /// Access to the request or the response.
    ///
    /// XXX: Signature will change to &self -> &Message
    fn virgin(&mut self) -> &mut H::MessageRef;

    /// Other side of the request/response pair, as compared to `virgin`.
    ///
    /// This will return `None` if the adapter is on the request side of
    /// a proxy, as there is no cause in that case.
    ///
    /// XXX: Signature will change to &self -> Option<&Message>
    fn cause(&mut self) -> &H::MessageRef;

    /// The message passed to `use_adapted`.
    ///
    /// This method will return `None` if the `use_adapted` method has
    /// not been called.
    ///
    /// XXX: Why does this return a reference to message and not the
    /// shared_ptr that is given to `use_adapted`?
    ///
    /// XXX: Signature will change to &self -> Option<&Message>
    fn adapted(&mut self) -> &mut H::MessageRef;

    /// Use the virgin message for response/request.
    ///
    /// This may be called even if the adapter wishes to examine message
    /// body, but to do so `virgin_body_make` must have been called before hand.
    ///
    /// Host will not call `adapted_body` methods on `adapter::Transaction`.
    fn use_virgin(&mut self);

    /// Use the message passed for response/request.
    ///
    /// By calling this, the adapter indicates that the host should call
    /// the `adapted_body` methods on the `adapter::Transaction` in
    /// order to receive a message body.
    fn use_adapted<M: Message<H> + 'static>(&mut self, msg: M);

    /// Prevent access to this message.
    ///
    /// If interest was registered beforehand via `virgin_body_make`
    /// adapter may still view the virgin body.
    ///
    /// Host will not call `adapted_body` methods on `adapter::Transaction`.
    fn block_virgin(&mut self);

    /// More time is needed to return adapted body to the transaction.
    ///
    /// This is intended for synchronous adapters. Callers indicate that
    /// the host should not expect new adapted content for some period
    /// of time.
    fn adaptation_delayed(&mut self, delay: &Delay);

    /// Adapter transaction terminated abnormally.
    ///
    /// Neither host nor adapter will call any additional methods.
    fn adaptation_aborted(&mut self);

    /// Register interest in resuming this transaction.
    ///
    /// Will eventually call `adapter::Transaction::resume` on the
    /// associated transaction.
    ///
    /// Must be called only by `Service::resume`.
    fn resume(&mut self);

    /// Adapter will not look at the virgin body.
    ///
    /// This is the opposite method to `virgin_body_make`.
    ///
    /// After calling this, the adapter must not call any other
    /// virgin_body methods.
    fn virgin_body_discard(&mut self);

    /// Adapter is interested in the virgin body.
    ///
    /// This is the opposite method to `virgin_body_discard`.
    ///
    /// Must be called only once.
    fn virgin_body_make(&mut self);

    /// Adapter requires more virgin body.
    ///
    /// Can be called repeatedly.
    fn virgin_body_make_more(&mut self);

    /// Adapter will no longer request virgin body content.
    ///
    /// Adapter should not call `virgin_body_content`.
    fn virgin_body_stop_making(&mut self);

    /// Pause making virgin body content.
    ///
    /// Unlike [`virgin_body_stop_making`][], this does not represent
    /// an end state, and body creation can be resumed via
    /// [`virgin_body_resume`]. Once creation is paused,
    /// [`virgin_body_resume`] will be called before any other methods.
    ///
    /// [`virgin_body_stop_making`]: `Transaction::virgin_body_stop_making`
    /// [`virgin_body_resume`]: `Transaction::virgin_body_resume`
    fn virgin_body_pause(&mut self);

    /// Resume making virgin body content.
    ///
    /// Will only be called after [`virgin_body_pause`][].
    ///
    /// [`virgin_body_pause`]: `Transaction::virgin_body_pause`
    fn virgin_body_resume(&mut self);

    /// Extract a given portion of the virgin body content.
    ///
    /// This may return less than the size requested, but must return
    /// content at the given offset.
    ///
    /// See [`adapter::Transaction::adapted_body_content`][] for
    /// further details.
    ///
    /// [`adapter::Transaction::adapted_body_content`]: `::adapter::Transaction::adapted_body_content`
    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area;

    /// Shift over start of content buffer.
    ///
    /// Future calls to `virgin_body_content` must pass offsets with
    /// respect to this shift.
    ///
    /// See [`adapter::Transaction::adapted_body_content_shift`][] for
    /// further details.
    ///
    /// [`adapter::Transaction::adapted_body_content_shift`]: `::adapter::Transaction::adapted_body_content_shift`
    fn virgin_body_content_shift(&mut self, size: usize);

    /// No more adapted body content is expected.
    ///
    /// The `at_end` argument indicates whether this termination is
    /// expected, or if the stream was cut off. For example, if a
    /// `Content-Length` header larger than the body was provided, `at_end`
    /// will be false. Despite that, no more body will be available.
    ///
    /// See [`adapter::Transaction::virgin_body_content_done`][] for
    /// further details.
    ///
    /// [`adapter::Transaction::virgin_body_content_done`]: `::adapter::Transaction::virgin_body_content_done`
    fn adapted_body_content_done(&mut self, at_end: bool);

    /// More adapted body content may be available.
    ///
    /// See [`adapter::Transaction::virgin_body_content_available`][] for
    /// further details.
    ///
    /// [`adapter::Transaction::virgin_body_content_available`]: `::adapter::Transaction::virgin_body_content_available`
    fn adapted_body_content_available(&mut self);
}

impl<H: Host + ?Sized, T: ?Sized + Transaction<H>> Transaction<H> for Box<T> {
    fn virgin(&mut self) -> &mut H::MessageRef {
        (&mut **self).virgin()
    }
    fn cause(&mut self) -> &H::MessageRef {
        (&mut **self).cause()
    }
    fn adapted(&mut self) -> &mut H::MessageRef {
        (&mut **self).adapted()
    }
    fn use_virgin(&mut self) {
        (&mut **self).use_virgin()
    }
    fn use_adapted<M: Message<H> + 'static>(&mut self, msg: M) {
        (&mut **self).use_adapted(msg)
    }
    fn block_virgin(&mut self) {
        (&mut **self).block_virgin()
    }
    fn adaptation_delayed(&mut self, delay: &Delay) {
        (&mut **self).adaptation_delayed(delay)
    }
    fn adaptation_aborted(&mut self) {
        (&mut **self).adaptation_aborted()
    }
    fn resume(&mut self) {
        (&mut **self).resume()
    }
    fn virgin_body_discard(&mut self) {
        (&mut **self).virgin_body_discard()
    }
    fn virgin_body_make(&mut self) {
        (&mut **self).virgin_body_make()
    }
    fn virgin_body_make_more(&mut self) {
        (&mut **self).virgin_body_make_more()
    }
    fn virgin_body_stop_making(&mut self) {
        (&mut **self).virgin_body_stop_making()
    }
    fn virgin_body_pause(&mut self) {
        (&mut **self).virgin_body_pause()
    }
    fn virgin_body_resume(&mut self) {
        (&mut **self).virgin_body_resume()
    }
    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
        (&mut **self).virgin_body_content(offset, size)
    }
    fn virgin_body_content_shift(&mut self, size: usize) {
        (&mut **self).virgin_body_content_shift(size)
    }
    fn adapted_body_content_done(&mut self, at_end: bool) {
        (&mut **self).adapted_body_content_done(at_end)
    }
    fn adapted_body_content_available(&mut self) {
        (&mut **self).adapted_body_content_available()
    }
}
