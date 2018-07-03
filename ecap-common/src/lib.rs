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
pub extern "C" fn register_service(service: *mut u32) {
    let service = unsafe { Box::from_raw(service as *mut ErasedService) };

    let translators = REGISTERED_TRANSLATORS.lock().unwrap();
    if let Some(translator) = translators.first() {
        translator.register_service(*service);
    } else {
        let mut adapters = REGISTERED_ADAPTERS.lock().unwrap();
        adapters.push(*service);
    }
}

#[no_mangle]
pub extern "C" fn register_translator(translator: *mut u32) {
    unsafe {
        let translator: Box<ErasedTranslatorS> =
            Box::from_raw(translator as *mut ErasedTranslatorS);

        let mut translators = REGISTERED_TRANSLATORS.lock().unwrap();
        translators.push(*translator);
    }
}
