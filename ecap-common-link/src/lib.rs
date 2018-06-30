extern crate ecap;
extern crate erased_ecap;

use erased_ecap::adapter::ErasedService;
use erased_ecap::host::Host;
use erased_ecap::ErasedTranslator;
use erased_ecap::ErasedTranslatorS;

use ecap::adapter::Service;

mod ffi {
    extern "C" {
        pub fn register_service(service: *mut u32);
        pub fn register_translator(translator: *mut u32);
    }
}

pub fn register_erased_service<T: Service<dyn Host>>(service: T)
where
    <T as Service<dyn Host>>::Transaction: 'static,
{
    unsafe {
        let service: Box<ErasedService> = Box::new(ErasedService::new(service));
        ffi::register_service(Box::into_raw(service) as *mut u32);
    }
}

pub fn register_erased_translator<T: 'static + ErasedTranslator>(translator: T) {
    unsafe {
        let translator = ErasedTranslatorS::new(translator);
        ffi::register_translator(Box::into_raw(Box::new(translator)) as *mut u32);
    }
}
