#[macro_use]
extern crate lazy_static;
extern crate ecap;

use ecap::adapter::Service;
use ecap::Translator;
use std::sync::Mutex;

lazy_static! {
    pub static ref REGISTERED_ADAPTERS: Mutex<Vec<Box<dyn Service + Send + Sync>>> =
        Mutex::new(Vec::new());
    pub static ref REGISTERED_TRANSLATORS: Mutex<Vec<Box<dyn Translator>>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub extern "C" fn register_service(service: *mut *mut u32) {
    let service = unsafe {
        let b = Box::from_raw(service as *mut *mut (dyn Service + Send + Sync));
        let service = Box::from_raw(*b);
        service
    };

    let translators = REGISTERED_TRANSLATORS.lock().unwrap();
    if let Some(translator) = translators.first() {
        println!("registered service directly");
        translator.register_service(service);
    } else {
        let mut adapters = REGISTERED_ADAPTERS.lock().unwrap();
        adapters.push(service);
        println!("registered service (adapters: {})", adapters.len());
    }
}

#[no_mangle]
pub extern "C" fn register_translator(translator: *mut *mut u32) {
    unsafe {
        let b = Box::from_raw(translator as *mut *mut dyn Translator);
        let translator = Box::from_raw(*b);
        let mut translators = REGISTERED_TRANSLATORS.lock().unwrap();
        if !translators.is_empty() {
            panic!("registered two translators!");
        }
        translators.push(translator);
        println!("registered translator (translators: {})", translators.len());
    }

    let translators = REGISTERED_TRANSLATORS.lock().unwrap();
    let mut adapters = REGISTERED_ADAPTERS.lock().unwrap();
    let translator = translators.first().unwrap();
    for service in adapters.drain(..) {
        translator.register_service(service);
    }
}
