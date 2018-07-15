#![feature(unwind_attributes)]

#[macro_use]
extern crate lazy_static;
extern crate ecap;
extern crate erased_ecap;

use erased_ecap::adapter::ErasedService;
use erased_ecap::ErasedTranslatorS;
use std::sync::Mutex;

lazy_static! {
    pub static ref REGISTERED_ADAPTERS: Mutex<Vec<ErasedService>> = Mutex::new(Vec::new());
    pub static ref REGISTERED_TRANSLATORS: Mutex<Vec<ErasedTranslatorS>> = Mutex::new(Vec::new());
}

#[no_mangle]
#[unwind(allowed)]
pub fn register_service(service: ErasedService) {
    let translators = REGISTERED_TRANSLATORS.lock().unwrap();
    if let Some(translator) = translators.first() {
        translator.register_service(service);
    } else {
        let mut adapters = REGISTERED_ADAPTERS.lock().unwrap();
        adapters.push(service);
    }
}

#[no_mangle]
#[unwind(allowed)]
pub fn register_translator(translator: ErasedTranslatorS) {
    let mut translators = REGISTERED_TRANSLATORS.lock().unwrap();
    translators.push(translator);
}
