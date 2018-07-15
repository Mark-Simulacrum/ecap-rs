#![feature(unwind_attributes)]

extern crate ecap;
extern crate erased_ecap;

use erased_ecap::adapter::ErasedService;
use erased_ecap::host::Host;
use erased_ecap::ErasedTranslator;
use erased_ecap::ErasedTranslatorS;

use ecap::adapter::Service;

#[allow(improper_ctypes)]
extern "Rust" {
    #[unwind(allowed)]
    fn register_service(service: ErasedService);
    #[unwind(allowed)]
    fn register_translator(translator: ErasedTranslatorS);
}

pub fn register_erased_service<T: Service<dyn Host>>(service: T)
where
    <T as Service<dyn Host>>::Transaction: 'static,
{
    unsafe {
        let service = ErasedService::new(service);
        register_service(service);
    }
}

pub fn register_erased_translator<T: 'static + ErasedTranslator>(translator: T) {
    unsafe {
        let translator = ErasedTranslatorS::new(translator);
        register_translator(translator);
    }
}
