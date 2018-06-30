#![feature(crate_in_paths, core_intrinsics)]

extern crate ecap;
#[macro_use]
extern crate mopa;
#[macro_use]
extern crate parse_generics_shim;

pub mod adapter;
pub mod common;
pub mod host;

use adapter::ErasedService;

pub trait ErasedTranslator {
    fn register_service(&self, s: ErasedService);
}

impl<T> ErasedTranslator for T
where
    T: ecap::Translator,
{
    fn register_service(&self, s: ErasedService) {
        let service = s.take::<dyn host::Host>();
        Self::register_service(self, service);
    }
}

pub struct ErasedTranslatorS {
    service: *mut dyn ErasedTranslator, // lies
}

impl ErasedTranslatorS {
    pub fn new<T: ErasedTranslator + 'static>(t: T) -> Self {
        ErasedTranslatorS {
            service: Box::into_raw(Box::new(t)),
        }
    }

    pub fn register_service(&self, service: ErasedService) {
        unsafe { (&*self.service).register_service(service) }
    }

    pub fn read(self) -> Box<dyn ErasedTranslator> {
        unsafe { Box::from_raw(self.service) }
    }
}

unsafe impl Send for ErasedTranslatorS {}
unsafe impl Sync for ErasedTranslatorS {}
