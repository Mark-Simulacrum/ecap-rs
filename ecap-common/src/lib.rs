#![feature(unwind_attributes)]
#![feature(alloc_system)]

extern crate alloc_system;

#[global_allocator]
static ALLOC: alloc_system::System = alloc_system::System;

#[macro_use]
extern crate lazy_static;
extern crate ecap;
extern crate erased_ecap;

use erased_ecap::adapter::ErasedService;
use erased_ecap::ErasedTranslatorS;
use std::sync::Mutex;

lazy_static! {
    pub static ref REGISTERED_ADAPTERS: Mutex<Vec<ErasedService>> = Mutex::new(Vec::new());
    pub static ref REGISTERED_TRANSLATORS: Mutex<Option<ErasedTranslatorS>> = Mutex::new(None);
}

#[no_mangle]
#[unwind(allowed)]
pub fn register_service(service: ErasedService) {
    let translator_slot = REGISTERED_TRANSLATORS.lock().unwrap();
    if let Some(translator) = &*translator_slot {
        translator.register_service(service);
    } else {
        let mut adapters = REGISTERED_ADAPTERS.lock().unwrap();
        adapters.push(service);
    }
}

#[no_mangle]
#[unwind(allowed)]
pub fn register_translator(translator: ErasedTranslatorS) {
    let mut adapters = REGISTERED_ADAPTERS.lock().unwrap();
    for adapter in adapters.drain(..) {
        translator.register_service(adapter);
    }
    let mut translator_slot = REGISTERED_TRANSLATORS.lock().unwrap();
    assert!(translator_slot.is_none());
    *translator_slot = Some(translator);
}
