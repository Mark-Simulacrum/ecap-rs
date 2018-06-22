use ecap;

pub mod shim {
    use std::mem;
    use std::borrow::Cow;
    use libc::{size_t, c_char, c_void};

    use ecap::Area;
    use message::{SharedPtrMessage, Message};
    use super::Transaction;
    use shim::{to_service_mut, ServicePtr};

    use ffi;

    foreign_ref!(pub struct HostTransaction(ffi::HostTransaction));

    #[derive(Debug)]
    // FIXME: This struct isn't quite right -- C++ permits having no fields, just state, or both
    pub struct Delay {
        state: Cow<'static, str>,
        progress: f64,
    }

    impl HostTransaction {
        accessor!(fn virgin() -> &mut Message: ffi::rust_shim_host_xaction_virgin);
        accessor!(fn cause(&mut self) -> &Message: ffi::rust_shim_host_xaction_cause);
        accessor!(fn adapted() -> &mut Message: ffi::rust_shim_host_xaction_adapted);

        pub fn use_virgin(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_use_virgin(self.as_ptr_mut());
            }
        }

        pub fn use_adapted(&mut self, msg: &SharedPtrMessage) {
            unsafe {
                ffi::rust_shim_host_xaction_use_adapted(self.as_ptr_mut(), msg.as_ptr());
            }
        }

        pub fn block_virgin(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_block_virgin(self.as_ptr_mut());
            }
        }

        pub fn adaptation_delayed(&mut self, delay: &Delay) {
            unsafe {
                ffi::rust_shim_host_xaction_adaptation_delayed(
                    self.as_ptr_mut(), delay.state.as_ptr() as *const c_char, delay.state.len(), delay.progress);
            }
        }

        pub fn adaptation_aborted(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_adaptation_aborted(self.as_ptr_mut());
            }
        }

        pub fn resume(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_resume(self.as_ptr_mut());
            }
        }

        pub fn virgin_body_discard(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_vb_discard(self.as_ptr_mut());
            }
        }

        pub fn virgin_body_make(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_vb_make(self.as_ptr_mut());
            }
        }

        pub fn virgin_body_stop_making(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_vb_stop_making(self.as_ptr_mut());
            }
        }

        pub fn virgin_body_make_more(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_vb_make_more(self.as_ptr_mut());
            }
        }

        pub fn virgin_body_pause(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_vb_pause(self.as_ptr_mut());
            }
        }

        pub fn virgin_body_resume(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_vb_resume(self.as_ptr_mut());
            }
        }

        pub fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
            unsafe {
                Area::from_raw(ffi::rust_shim_host_xaction_vb_content(self.as_ptr_mut(), offset, size))
            }
        }

        pub fn virgin_body_content_shift(&mut self, size: usize) {
            unsafe {
                ffi::rust_shim_host_xaction_vb_content_shift(self.as_ptr_mut(), size)
            }
        }

        pub fn note_adapted_body_content_done(&mut self, at_end: bool) {
            unsafe {
                ffi::rust_shim_host_xaction_note_ab_content_done(self.as_ptr_mut(), at_end);
            }
        }

        pub fn note_adapted_body_content_available(&mut self) {
            unsafe {
                ffi::rust_shim_host_xaction_note_ab_content_available(self.as_ptr_mut());
            }
        }
    }

    type TransactionPtr = *mut *mut c_void;

    #[allow(unused)]
    unsafe fn to_transaction<'a>(transaction: &'a TransactionPtr) -> &'a dyn Transaction {
        assert!(!transaction.is_null());
        let transaction: *mut *mut dyn Transaction = mem::transmute(*transaction);
        let transaction = *(transaction as *mut *mut dyn Transaction);
        &*transaction
    }

    unsafe fn to_transaction_mut<'a>(transaction: &'a mut TransactionPtr) -> &'a mut dyn Transaction {
        assert!(!transaction.is_null());
        let transaction: *mut *mut dyn Transaction = mem::transmute(*transaction);
        let transaction = *(transaction as *mut *mut dyn Transaction);
        &mut *transaction
    }

    macro_rules! transaction_mut_method {
        ($c:ident, $method:ident) => {
            #[no_mangle]
            pub unsafe extern "C" fn $c(mut data: TransactionPtr) {
                to_transaction_mut(&mut data).$method();
            }
        }
    }

    transaction_mut_method!(rust_xaction_start, start);
    transaction_mut_method!(rust_xaction_stop, stop);
    transaction_mut_method!(rust_xaction_resume, resume);
    transaction_mut_method!(rust_xaction_ab_discard, adapted_body_discard);
    transaction_mut_method!(rust_xaction_ab_make, adapted_body_make);
    transaction_mut_method!(rust_xaction_ab_make_more, adapted_body_make_more);
    transaction_mut_method!(rust_xaction_ab_stop_making, adapted_body_stop_making);
    transaction_mut_method!(rust_xaction_ab_pause, adapted_body_pause);
    transaction_mut_method!(rust_xaction_ab_resume, adapted_body_resume);
    transaction_mut_method!(rust_xaction_vb_content_available, virgin_body_content_available);

    #[no_mangle]
    pub unsafe extern "C" fn rust_xaction_ab_content(
        mut data: TransactionPtr,
        offset: size_t,
        size: size_t,
    ) -> ffi::Area {
        to_transaction_mut(&mut data).adapted_body_content(offset, size).raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn rust_xaction_ab_content_shift(
        mut data: TransactionPtr,
        size: size_t,
    ) {
        to_transaction_mut(&mut data).adapted_body_content_shift(size);
    }

    #[no_mangle]
    pub unsafe extern "C" fn rust_xaction_vb_content_done(mut data: TransactionPtr, at_end: bool) {
        to_transaction_mut(&mut data).virgin_body_content_done(at_end);
    }

    #[no_mangle]
    pub unsafe extern fn rust_xaction_create(mut service: ServicePtr, host: *mut HostTransaction) -> TransactionPtr {
        let service = to_service_mut(&mut service);
        let transaction = service.make_transaction(host);
        let ptr = Box::into_raw(transaction);
        let transaction_ptr: Box<*mut dyn Transaction> = Box::new(ptr);
        Box::into_raw(transaction_ptr) as *mut *mut c_void
    }

    #[no_mangle]
    pub unsafe extern fn rust_xaction_free(transaction: TransactionPtr) {
        assert!(!transaction.is_null());
        let ptr: Box<*mut dyn Transaction> = Box::from_raw(transaction as *mut *mut dyn Transaction);
        let tr: Box<dyn Transaction> = Box::from_raw(*ptr);
        mem::drop(tr);
    }
}

pub trait Transaction {
    fn start(&mut self);
    fn stop(&mut self);
    fn resume(&mut self);
    fn adapted_body_discard(&mut self);
    fn adapted_body_make(&mut self);
    fn adapted_body_make_more(&mut self);
    fn adapted_body_stop_making(&mut self);
    fn adapted_body_pause(&mut self);
    fn adapted_body_resume(&mut self);

    fn adapted_body_content(&mut self, offset: usize, size: usize) -> ecap::Area;
    fn adapted_body_content_shift(&mut self, size: usize);

    fn virgin_body_content_done(&mut self, at_end: bool);
    fn virgin_body_content_available(&mut self);
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Undecided,
    On,
    Complete,
    Never,
}

pub struct Minimal<'a> {
    pub host: Option<&'a mut HostTransaction>,
    pub receiving: State,
    pub sending: State,
}

pub use xaction::shim::HostTransaction;

macro_rules! host {
    ($s:expr) => {
        $s.host.as_mut().unwrap()
    }
}

impl<'a> Transaction for Minimal<'a> {
    fn start(&mut self) {
        println!("starting xaction");
        println!("version = {:?}", host!(self).virgin().first_line().version());
        println!("body = {}", host!(self).virgin().body().is_some());
        if host!(self).virgin().body().is_some() {
            self.receiving = State::On;
            host!(self).virgin_body_make();
            host!(self).virgin().header().visit_each(|name, value| {
                println!("header: {:?}: {:?}", name, value);
            });
            println!("body size = {:?}", host!(self).virgin().body().unwrap().size());
        } else {
            self.receiving = State::Never;
        }

        let adapted = host!(self).virgin().clone();
        if adapted.body().is_none() {
            self.sending = State::Never; // nothing to send
            host!(self).use_adapted(&adapted);
            println!("set host to none");
            self.host = None;
        } else {
            host!(self).use_adapted(&adapted);
        }
    }

    fn stop(&mut self) {
        let _ = self.host.take();
        println!("stopping xaction");
    }

    fn resume(&mut self) { }

    fn adapted_body_discard(&mut self) {
        assert_eq!(self.sending, State::Undecided);
        self.sending = State::Never;
    }

    fn adapted_body_make(&mut self) {
        assert_eq!(self.sending, State::Undecided);
        assert!(host!(self).virgin().body().is_some());
        assert!(self.receiving == State::On || self.receiving == State::Complete);

        self.sending = State::On;
        host!(self).note_adapted_body_content_available();
    }

    fn adapted_body_make_more(&mut self) {
        assert_eq!(self.receiving, State::On);
        host!(self).virgin_body_make_more();
    }

    fn adapted_body_stop_making(&mut self) {
        self.sending = State::Complete;
    }

    fn adapted_body_pause(&mut self) {}
    fn adapted_body_resume(&mut self) {}

    fn adapted_body_content(&mut self, offset: usize, size: usize) -> ecap::Area {
        assert_eq!(self.sending, State::On);
        host!(self).virgin_body_content(offset, size)
    }

    fn adapted_body_content_shift(&mut self, offset: usize) {
        assert_eq!(self.sending, State::On);
        host!(self).virgin_body_content_shift(offset);
    }

    fn virgin_body_content_done(&mut self, at_end: bool) {
        assert_eq!(self.receiving, State::On);
        host!(self).virgin_body_stop_making();
        self.receiving = State::Complete;
        host!(self).note_adapted_body_content_done(at_end);
    }

    fn virgin_body_content_available(&mut self) {
        assert_eq!(self.receiving, State::On);
        if self.sending == State::On {
            host!(self).note_adapted_body_content_available();
        }
    }
}

impl<'a> Drop for Minimal<'a> {
    fn drop(&mut self) {
        if let Some(host) = self.host.take() {
            host.adaptation_aborted();
        }
        println!("dropping minimal xaction!");
    }
}
