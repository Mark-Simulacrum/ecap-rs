use std::any::TypeId;
use std::ffi::CStr;
use std::time::Duration;

use std::intrinsics::type_name;

use ecap;

use adapter;
use common;
use host;

use host::Host as ErasedHost;
use host::Transaction as ErasedTransaction;

pub struct ErasedService {
    service: *mut dyn Service<Box<dyn host::Host>>, // lies
    host: TypeId,
}

unsafe impl Send for ErasedService {}
unsafe impl Sync for ErasedService {}

impl ErasedService {
    pub fn new<H: ?Sized + host::Host + 'static, S: Service<H>>(s: S) -> ErasedService {
        unsafe {
            println!(
                "erasing service {:?} with host {:?}",
                type_name::<S>(),
                type_name::<H>()
            );
        }
        ErasedService {
            service: Box::into_raw(Box::new(s)) as *mut dyn Service<H>
                as *mut dyn Service<Box<dyn host::Host>>,
            host: TypeId::of::<H>(),
        }
    }

    pub fn take<H: ?Sized + host::Host + 'static>(self) -> Box<dyn Service<H>> {
        if TypeId::of::<H>() == self.host {
            unsafe { Box::from_raw(self.service as *mut dyn Service<H>) }
        } else {
            panic!("taking with a different host: {}", unsafe {
                type_name::<H>()
            })
        }
    }
}

pub trait Service<H: 'static + host::Host + ?Sized> {
    fn uri(&self) -> String;
    fn tag(&self) -> String;
    fn describe(&self) -> String;
    fn configure(&mut self, options: &dyn common::Options);
    fn reconfigure(&mut self, options: &dyn common::Options);
    fn start(&self);
    fn stop(&self);
    fn retire(&self);
    fn wants_url(&self, url: &CStr) -> bool;
    fn make_transaction<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<H> + 'static),
    ) -> Box<dyn adapter::Transaction>;
    fn is_async(&self) -> bool;
    fn suspend(&self, _timeout: &mut Duration);
    fn resume(&self);
}

impl<S> Service<dyn ErasedHost> for S
where
    S: ecap::adapter::Service<dyn ErasedHost> + ?Sized,
    <S as ecap::adapter::Service<dyn ErasedHost>>::Transaction: 'static,
{
    fn make_transaction(
        &mut self,
        host: &mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    ) -> Box<dyn adapter::Transaction> {
        Box::new(S::make_transaction(self, host))
    }

    fn uri(&self) -> String {
        S::uri(self)
    }

    fn tag(&self) -> String {
        S::tag(self)
    }

    fn describe(&self) -> String {
        S::describe(self)
    }

    fn configure(&mut self, options: &dyn common::Options) {
        S::configure(self, &options)
    }

    fn reconfigure(&mut self, options: &dyn common::Options) {
        S::reconfigure(self, &options)
    }

    fn start(&self) {
        S::start(self)
    }

    fn stop(&self) {
        S::stop(self)
    }

    fn retire(&self) {
        S::retire(self)
    }

    fn wants_url(&self, url: &CStr) -> bool {
        S::wants_url(self, url)
    }

    fn is_async(&self) -> bool {
        S::is_async(self)
    }

    fn suspend(&self, timeout: &mut Duration) {
        S::suspend(self, timeout)
    }

    fn resume(&self) {
        S::resume(self)
    }
}

impl ecap::adapter::Service<dyn ErasedHost> for dyn Service<dyn ErasedHost> {
    type Transaction = Box<dyn adapter::Transaction>;
    fn uri(&self) -> String {
        Self::uri(self)
    }
    fn tag(&self) -> String {
        Self::tag(self)
    }
    fn describe(&self) -> String {
        Self::describe(self)
    }
    fn configure<T: ecap::common::Options>(&mut self, options: &T) {
        Self::configure(self, options)
    }
    fn reconfigure<T: ecap::common::Options>(&mut self, options: &T) {
        Self::reconfigure(self, options)
    }
    fn start(&self) {
        Self::start(self)
    }
    fn stop(&self) {
        Self::stop(self)
    }
    fn retire(&self) {
        Self::retire(self)
    }
    fn wants_url(&self, url: &CStr) -> bool {
        Self::wants_url(self, url)
    }
    fn make_transaction<'a>(
        &mut self,
        host: &'a mut (dyn ErasedTransaction<dyn ErasedHost> + 'static),
    ) -> Box<dyn adapter::Transaction> {
        Self::make_transaction(self, host)
    }
    fn is_async(&self) -> bool {
        Self::is_async(self)
    }
    fn suspend(&self, timeout: &mut Duration) {
        Self::suspend(self, timeout)
    }
    fn resume(&self) {
        Self::resume(self)
    }
}
