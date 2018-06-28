extern crate ecap;

use ecap::adapter::Service;
use ecap::Translator;

mod ffi {
    extern "C" {
        pub fn register_service(service: *mut *mut u32);
        pub fn register_translator(service: *mut *mut u32);
    }
}

pub fn register_service<T: Service + Sync + Send>(service: T) {
    unsafe {
        let service: Box<dyn Service> = Box::new(service);
        ffi::register_service(
            Box::into_raw(Box::new(Box::into_raw(service)))
            as *mut *mut u32);
    }
}

pub fn register_translator<T: Translator>(translator: T) {
    unsafe {
        let translator: Box<dyn Translator> = Box::new(translator);
        ffi::register_translator(
            Box::into_raw(Box::new(Box::into_raw(translator)))
            as *mut *mut u32);
    }
}
