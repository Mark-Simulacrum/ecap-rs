use std::ffi::CStr;
use std::time::Duration;

use {adapter, common::Options, host};

/// This trait is the equivalent of libecap::adapter::Service.
pub trait Service<H: ?Sized + host::Host> {
    type Transaction: adapter::Transaction<H>;

    /// The returned string should be unique across vendors.
    fn uri(&self) -> String;

    /// Identifies this version and configuration of this adapter: the
    /// URI and tag should uniquely identify a given adapter.
    fn tag(&self) -> String;

    /// Free-format description of the adapter
    // FIXME: Migrate to fmt::Display impl?
    fn describe(&self) -> String;

    /// Determines whether this adapter requires async transactions.
    ///
    /// If false (as by default) the Host will not call suspend and resume.
    fn is_async(&self) -> bool {
        false
    }

    /// Called by the Host to initially configure the adapter service.
    ///
    /// Should only be called once.
    fn configure<T: Options>(&mut self, options: &T);

    /// Called by the host when the configuration for an adapter
    /// changes. It may be called with the same configuration as passed
    /// previously.
    fn reconfigure<T: Options>(&mut self, options: &T);

    /// Prepare for creation of transactions via `make_transaction` calls.
    fn start(&self);

    /// Provides a hint to the host as to how soon transactions will be
    /// ready for further processing. It is invalid to increase the
    /// Duration passed. Implementations which wish to be called after
    /// that timeout should not modify the value passed.
    ///
    /// This method does not guarantee that the host will call `resume`
    /// after the timeout specified: it may be called sooner, never, or
    /// later.
    ///
    /// Only called for async services.
    fn suspend(&self, _timeout: &mut Duration) {
        unimplemented!("Service::suspend is not implemented for this async adapter");
    }

    /// If this is an async service, then this method should
    /// call `host::Transaction::resume` on any transactions which
    /// the host should resume processing.
    ///
    /// Note that it *must not* call any other methods on host::Transaction.
    fn resume(&self) {
        unimplemented!("Service::suspend is not implemented for this async adapter");
    }

    /// Pause making transactions until `start` is called. Note that it
    /// may not be called.
    fn stop(&self);

    /// Host will not make further calls to `make_transaction`.
    fn retire(&self);

    /// Should make_transaction be called?
    ///
    /// Services which only need to examine a subset of transactions,
    /// and can determine this based on the URL, can use this method
    /// to increase their performance.
    fn wants_url(&self, url: &CStr) -> bool;

    /// Create a transaction to give to the Host.
    fn make_transaction(&mut self, host: &mut H::TransactionRef) -> Self::Transaction;

    // FIXME: libecap API also exposes a shared_ptr to self in public
    // API
}

impl<H, S: Service<H> + ?Sized> Service<H> for Box<S>
where
    H: ?Sized,
    H: host::Host,
{
    type Transaction = S::Transaction;

    fn uri(&self) -> String {
        (&**self).uri()
    }
    fn tag(&self) -> String {
        (&**self).tag()
    }
    fn describe(&self) -> String {
        (&**self).describe()
    }
    fn configure<T: Options>(&mut self, options: &T) {
        (&mut **self).configure(options)
    }
    fn reconfigure<T: Options>(&mut self, options: &T) {
        (&mut **self).reconfigure(options)
    }
    fn start(&self) {
        (&**self).start()
    }
    fn stop(&self) {
        (&**self).stop()
    }
    fn retire(&self) {
        (&**self).retire()
    }
    fn wants_url(&self, url: &CStr) -> bool {
        (&**self).wants_url(url)
    }
    fn make_transaction(&mut self, host: &mut H::TransactionRef) -> Self::Transaction {
        (&mut **self).make_transaction(host)
    }

    fn is_async(&self) -> bool {
        (&**self).is_async()
    }
    fn suspend(&self, timeout: &mut Duration) {
        (&**self).suspend(timeout)
    }
    fn resume(&self) {
        (&**self).resume()
    }
}
