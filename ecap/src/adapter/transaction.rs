use common::{Area, Options};
use host;

/// Equivalent of libecap/adapter/xaction.h
///
/// This trait describes the adapatation of a single message from the
/// virgin state to the adapted state.
///
/// Implementations are created via Service::make_transaction and are
/// dropped by the host either before the call to `start` or after
/// `stop`.
///
/// All methods on this are intended only for calling by the host.
///
/// Transactions must also implement `Options` so that hosts can visit
/// meta-information from them.
///
/// XXX: What is the meta information?
pub trait Transaction<H: ?Sized + host::Host>: Options {
    /// Called by the host to initiate processing of the virgin request.
    ///
    /// XXX: Confirm that options methods can't be called prior to
    /// start.
    ///
    /// This will be called prior to any other methods on Transaction by
    /// the host, after creation in
    /// [`Service::make_transaction`](`::adapter::Service::make_transaction`).
    fn start<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// Processing has finished.
    ///
    /// No further calls to the host transaction should be made. The
    /// host transaction will also call no more methods on this adapter
    /// transaction.
    fn stop<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// Indicate readiness to provide content or otherwise change
    /// transaction state to the host transaction.
    ///
    /// When called, this method should indicate to the host readiness
    /// to provide content or a similar state change.
    ///
    /// This will be called eventually after `Service::resume` calls
    /// `host::Transaction::resume` on the host transaction pair to this
    /// transaction.
    fn resume<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// Discard the adapted body.
    ///
    /// This is called only before [`adapted_body_make`][].
    ///
    /// Note that this will only be called if the adapter has previously
    /// registered that it will provide the body via
    /// [`host::Transaction::use_adapted`][]. It will not be called
    /// after calling to [`host::Transaction::use_virgin`][], as that
    /// call does not signify intent to provide an adapted body.
    ///
    /// [`adapted_body_make`]: `Transaction::adapted_body_make`
    /// [`host::Transaction::use_adapted`]: `::host::Transaction::use_adapted`
    /// [`host::Transaction::use_virgin`]: `::host::Transaction::use_virgin`
    fn adapted_body_discard<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// The host is interested in adapted body content.
    ///
    /// This method will not be called more than once.
    ///
    /// Does not guarantee that the transaction will have content ready,
    /// rather provides an early hint that body will be required.
    ///
    /// This registers interest: while implementations may choose to
    /// begin creating a body here, they may also wait until a call to
    /// [`adapted_body_make_more`][].
    ///
    /// [`adapted_body_make_more`]: `Transaction::adapted_body_make_more`
    fn adapted_body_make<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// Make adapted body content.
    ///
    /// The host needs this transaction to return more body content in
    /// order to make progress.
    ///
    /// See also the [`adapted_body_make`][] method.
    ///
    /// [`adapted_body_make`]: `Transaction::adapted_body_make`
    fn adapted_body_make_more<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// Stop making adapted body content.
    ///
    /// The host will no longer need the adapted body. This method is
    /// only called after [`adapted_body_make`][]. No further calls to
    /// `adapted_body_*` methods will be made.
    ///
    /// Host will not call `adapted_body_content`.
    ///
    /// The difference between this method and
    /// [`adapted_body_discard`][] is that discard will only be called
    /// prior to [`adapted_body_make`][], whereas this is the reverse,
    /// called only after we've started making the body.
    ///
    /// [`adapted_body_make`]: `Transaction::adapted_body_make`
    /// [`adapted_body_discard`]: `Transaction::adapted_body_discard`
    fn adapted_body_stop_making<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// Pause making adapted body content.
    ///
    /// Unlike [`adapted_body_stop_making`][], this does not represent
    /// an end state, and body creation can be resumed via
    /// [`adapted_body_resume`]. Once this is called,
    /// [`adapted_body_resume`] will be called before any other methods.
    ///
    /// [`adapted_body_stop_making`]: `Transaction::adapted_body_stop_making`
    /// [`adapted_body_resume`]: `Transaction::adapted_body_resume`
    fn adapted_body_pause<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// Resume making adapted body content.
    ///
    /// Will only be called after [`adapted_body_pause`][].
    ///
    /// [`adapted_body_pause`]: `Transaction::adapted_body_pause`
    fn adapted_body_resume<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;

    /// Extract a given portion of the adapted body content.
    ///
    /// This may return less than the size requested. Hosts may request
    /// a size that is larger than the total message size. It must
    /// return a constant view and if data at a given offset was
    /// returned, it must always be returned.
    ///
    /// Note that the overall size of the message may be larger than
    /// possible for `usize`. The adapter must store the offset
    /// and size internally as a platform-independent type (e.g. `u64`).
    ///
    /// Adapters can assume that message size will not exceed the
    /// maximum value of an unsigned 64-bit integer.
    fn adapted_body_content<'a>(
        &mut self,
        host: &'a mut H::TransactionRef,
        offset: usize,
        size: usize,
    ) -> Area
    where
        H::TransactionRef: 'a;

    /// Shift over start of content buffer.
    ///
    /// As with [`adapted_body_content`][], note that the total offset
    /// over a transaction's lifetime may be larger than
    /// `usize::max_value()` and as such should be stored in `u64` or a
    /// larger type.
    ///
    /// Future calls to [`adapted_body_content`][] will pass offsets with
    /// respect to this shift.
    ///
    /// [`adapted_body_content`]: `Transaction::adapted_body_content`]
    fn adapted_body_content_shift<'a>(&mut self, host: &'a mut H::TransactionRef, size: usize)
    where
        H::TransactionRef: 'a;

    /// No more virgin body content is expected.
    ///
    /// The `at_end` argument indicates whether this termination is
    /// expected, or if the stream was cut off. For example, if a
    /// `Content-Length` header larger than the body was sent, `at_end`
    /// will be false. Despite that, no more body will be available.
    ///
    /// This method indicates fact, not a hint. No more content can
    /// arrive.
    fn virgin_body_content_done<'a>(&mut self, host: &'a mut H::TransactionRef, at_end: bool)
    where
        H::TransactionRef: 'a;

    /// More virgin body content may be available.
    ///
    /// Adapters are permitted to attempt to retrieve the virgin body
    /// content in this method.
    ///
    /// This method may be called and no additional virgin body content
    /// may be returned: it does not represent fact, merely a hint.
    fn virgin_body_content_available<'a>(&mut self, host: &'a mut H::TransactionRef)
    where
        H::TransactionRef: 'a;
}

macro_rules! generate_method_transaction {
    ($name:ident) => {
        fn $name<'a>(&mut self, host: &'a mut H::TransactionRef)
        where
            H::TransactionRef: 'a,
        {
            (&mut **self).$name(host)
        }
    }
}

impl<H, T> Transaction<H> for Box<T>
where
    H: ?Sized + host::Host,
    T: Options + Transaction<H> + ?Sized,
{
    generate_method_transaction!(start);
    generate_method_transaction!(stop);
    generate_method_transaction!(resume);
    generate_method_transaction!(adapted_body_discard);
    generate_method_transaction!(adapted_body_make);
    generate_method_transaction!(adapted_body_make_more);
    generate_method_transaction!(adapted_body_stop_making);
    generate_method_transaction!(adapted_body_pause);
    generate_method_transaction!(adapted_body_resume);

    fn adapted_body_content<'a>(
        &mut self,
        host: &'a mut H::TransactionRef,
        offset: usize,
        size: usize,
    ) -> Area
    where
        H::TransactionRef: 'a,
    {
        (&mut **self).adapted_body_content(host, offset, size)
    }
    fn adapted_body_content_shift<'a>(&mut self, host: &'a mut H::TransactionRef, size: usize)
    where
        H::TransactionRef: 'a,
    {
        (&mut **self).adapted_body_content_shift(host, size)
    }
    fn virgin_body_content_done<'a>(&mut self, host: &'a mut H::TransactionRef, at_end: bool)
    where
        H::TransactionRef: 'a,
    {
        (&mut **self).virgin_body_content_done(host, at_end)
    }

    generate_method_transaction!(virgin_body_content_available);
}
