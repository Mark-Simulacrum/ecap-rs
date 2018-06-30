use ffi;
use libc::{c_char, c_void, size_t};
use std::{mem, slice};

use ecap::common::{Area, Delay};

use common::message::{CppMessage, SharedPtrMessage};
use common::CppArea;

use host::CppHost;

use adapter::service::{to_service_mut, ServicePtr};
use ecap::common::Message as ConcreteMessage;
use erased_ecap::adapter::Transaction as ErasedAdapterTransaction;
use erased_ecap::common::Message as ErasedMessage;
use erased_ecap::host::Host as ErasedHost;
use erased_ecap::host::Transaction as ErasedTransaction;

//use ecap::host::Host as ConcreteHost;
use ecap::host::Transaction as ConcreteTransaction;

//foreign_ref!(pub struct Transaction(ffi::HostTransaction));
pub struct CppTransaction {
    hostx: *mut ffi::HostTransaction,
}

pub type CppTransactionRef = CppTransaction;

impl CppTransaction {
    fn from_ptr_mut(ptr: *mut ffi::HostTransaction) -> Self {
        CppTransaction { hostx: ptr }
    }

    fn as_ptr_mut(&mut self) -> *mut ffi::HostTransaction {
        self.hostx as *mut ffi::HostTransaction
    }
}

impl ErasedTransaction<dyn ErasedHost> for CppTransaction {
    fn virgin(&mut self) -> &mut dyn ErasedMessage {
        <CppTransaction as ConcreteTransaction<CppHost>>::virgin(self)
    }
    fn cause(&mut self) -> &dyn ErasedMessage {
        <CppTransaction as ConcreteTransaction<CppHost>>::cause(self)
    }
    fn adapted(&mut self) -> &mut dyn ErasedMessage {
        <CppTransaction as ConcreteTransaction<CppHost>>::adapted(self)
    }
    fn use_virgin(&mut self) {
        <CppTransaction as ConcreteTransaction<CppHost>>::use_virgin(self)
    }
    fn use_adapted(&mut self, msg: Box<dyn ErasedMessage>) {
        match msg.downcast::<Box<dyn ErasedMessage>>() {
            Ok(msg) => match msg.downcast::<SharedPtrMessage>() {
                Ok(msg) => {
                    <Self as ConcreteTransaction<CppHost>>::use_adapted::<SharedPtrMessage>(
                        self, *msg,
                    );
                }
                Err(_) => panic!("use_adapted should be called with result of clone"),
            },
            Err(_) => panic!("use_adapted should be called with result of clone, boxed once"),
        }
    }
    fn block_virgin(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::block_virgin(self)
    }
    fn adaptation_delayed(&mut self, delay: &Delay) {
        <Self as ConcreteTransaction<CppHost>>::adaptation_delayed(self, delay)
    }

    fn adaptation_aborted(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::adaptation_aborted(self)
    }

    fn resume(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::resume(self)
    }

    fn virgin_body_discard(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::virgin_body_make_more(self)
    }

    fn virgin_body_make(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::virgin_body_make(self)
    }

    fn virgin_body_make_more(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::virgin_body_make_more(self)
    }

    fn virgin_body_stop_making(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::virgin_body_stop_making(self)
    }

    fn virgin_body_pause(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::virgin_body_pause(self)
    }

    fn virgin_body_resume(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::virgin_body_resume(self)
    }

    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
        <Self as ConcreteTransaction<CppHost>>::virgin_body_content(self, offset, size)
    }

    fn virgin_body_content_shift(&mut self, size: usize) {
        <Self as ConcreteTransaction<CppHost>>::virgin_body_content_shift(self, size)
    }

    fn adapted_body_content_done(&mut self, at_end: bool) {
        <Self as ConcreteTransaction<CppHost>>::adapted_body_content_done(self, at_end)
    }

    fn adapted_body_content_available(&mut self) {
        <Self as ConcreteTransaction<CppHost>>::adapted_body_content_available(self)
    }
}

impl ConcreteTransaction<CppHost> for CppTransaction {
    fn virgin(&mut self) -> &mut CppMessage {
        unsafe { CppMessage::from_ptr_mut(ffi::rust_shim_host_xaction_virgin(self.as_ptr_mut())) }
    }
    fn cause(&mut self) -> &CppMessage {
        unimplemented!()
        //unsafe { CppMessage::from_ptr(ffi::rust_shim_host_xaction_cause(self.as_ptr_mut())) }
    }
    fn adapted(&mut self) -> &mut CppMessage {
        unimplemented!()
        //unsafe { CppMessage::from_ptr_mut(ffi::rust_shim_host_xaction_adapted(self.as_ptr_mut())) }
    }
    fn use_virgin(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_use_virgin(self.as_ptr_mut());
        }
    }
    fn use_adapted<M: 'static + ConcreteMessage<CppHost>>(&mut self, msg: M) {
        let v: &::std::any::Any = &msg;
        if let Some(shared_ptr_ref) = v.downcast_ref::<SharedPtrMessage>() {
            unsafe {
                ffi::rust_shim_host_xaction_use_adapted(
                    self.as_ptr_mut(),
                    <SharedPtrMessage>::as_ptr(shared_ptr_ref),
                );
            }
        } else {
            panic!("CppTransaction only works with Box<SharedPtrMessage>");
        }
    }
    fn block_virgin(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_block_virgin(self.as_ptr_mut());
        }
    }
    fn adaptation_delayed(&mut self, delay: &Delay) {
        let description = delay.description.as_ref().map(|s| s.as_ref()).unwrap_or("");
        unsafe {
            ffi::rust_shim_host_xaction_adaptation_delayed(
                self.as_ptr_mut(),
                description.as_ptr() as *const c_char,
                description.len(),
                delay.progress.unwrap_or(-1.0),
            );
        }
    }

    fn adaptation_aborted(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_adaptation_aborted(self.as_ptr_mut());
        }
    }

    fn resume(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_resume(self.as_ptr_mut());
        }
    }

    fn virgin_body_discard(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_vb_discard(self.as_ptr_mut());
        }
    }

    fn virgin_body_make(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_vb_make(self.as_ptr_mut());
        }
    }

    fn virgin_body_make_more(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_vb_make_more(self.as_ptr_mut());
        }
    }

    fn virgin_body_stop_making(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_vb_stop_making(self.as_ptr_mut());
        }
    }

    fn virgin_body_pause(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_vb_pause(self.as_ptr_mut());
        }
    }

    fn virgin_body_resume(&mut self) {
        unsafe {
            ffi::rust_shim_host_xaction_vb_resume(self.as_ptr_mut());
        }
    }

    fn virgin_body_content(&mut self, offset: usize, size: usize) -> Area {
        unsafe {
            let area = ffi::rust_shim_host_xaction_vb_content(self.as_ptr_mut(), offset, size);
            Area::new(CppArea::from_raw(area))
        }
    }

    fn virgin_body_content_shift(&mut self, size: usize) {
        unsafe { ffi::rust_shim_host_xaction_vb_content_shift(self.as_ptr_mut(), size) }
    }

    fn adapted_body_content_done(&mut self, at_end: bool) {
        unsafe { ffi::rust_shim_host_xaction_note_ab_content_done(self.as_ptr_mut(), at_end) }
    }

    fn adapted_body_content_available(&mut self) {
        unsafe { ffi::rust_shim_host_xaction_note_ab_content_available(self.as_ptr_mut()) }
    }
}

type TransactionPtr = *mut *mut c_void;

use erased_ecap::adapter::Transaction as AdapterTransaction;
//use erased_ecap::adapter::Transaction as ErasedTransaction;

#[allow(unused)]
unsafe fn to_transaction<'a>(transaction: &'a TransactionPtr) -> &'a dyn AdapterTransaction {
    //unimplemented!()
    assert!(!transaction.is_null());
    let transaction: *mut *mut dyn AdapterTransaction = mem::transmute(*transaction);
    let transaction = *(transaction as *mut *mut dyn AdapterTransaction);
    &*transaction
}

unsafe fn to_transaction_mut<'a>(
    transaction: &'a mut TransactionPtr,
) -> &'a mut dyn AdapterTransaction {
    //unimplemented!()
    assert!(!transaction.is_null());
    let transaction: *mut *mut dyn AdapterTransaction = mem::transmute(*transaction);
    let transaction = *(transaction as *mut *mut dyn AdapterTransaction);
    &mut *transaction
}

macro_rules! transaction_mut_method {
    ($c:ident, $method:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $c(mut data: TransactionPtr, host: *mut ffi::HostTransaction) {
            let mut host = CppTransactionRef::from_ptr_mut(host);
            to_transaction_mut(&mut data).$method(&mut host);
        }
    };
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
transaction_mut_method!(
    rust_xaction_vb_content_available,
    virgin_body_content_available
);

#[no_mangle]
pub unsafe extern "C" fn rust_xaction_ab_content(
    mut data: TransactionPtr,
    host: *mut ffi::HostTransaction,
    offset: size_t,
    size: size_t,
) -> ffi::Area {
    let mut host = CppTransactionRef::from_ptr_mut(host);
    let area = CppArea::from_area(
        to_transaction_mut(&mut data).adapted_body_content(&mut host, offset, size),
    );
    area.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn rust_xaction_ab_content_shift(
    mut data: TransactionPtr,
    host: *mut ffi::HostTransaction,
    size: size_t,
) {
    let mut host = CppTransactionRef::from_ptr_mut(host);
    to_transaction_mut(&mut data).adapted_body_content_shift(&mut host, size);
}

#[no_mangle]
pub unsafe extern "C" fn rust_xaction_vb_content_done(
    mut data: TransactionPtr,
    host: *mut ffi::HostTransaction,
    at_end: bool,
) {
    let mut host = CppTransactionRef::from_ptr_mut(host);
    to_transaction_mut(&mut data).virgin_body_content_done(&mut host, at_end);
}

#[no_mangle]
pub unsafe extern "C" fn rust_xaction_create(
    mut service: ServicePtr,
    host: *mut ffi::HostTransaction,
) -> TransactionPtr {
    let mut host = CppTransaction::from_ptr_mut(host);
    let service = to_service_mut(&mut service);
    //let mut host: Box<dyn ErasedTransaction<dyn ErasedHost>> = Box::new(host);
    let transaction: Box<dyn ErasedAdapterTransaction> = service.make_transaction(&mut host);
    let transaction_ptr = Box::new(transaction);

    Box::into_raw(transaction_ptr) as TransactionPtr
}

#[no_mangle]
pub unsafe extern "C" fn rust_xaction_free(transaction: TransactionPtr) {
    assert!(!transaction.is_null());
    let _: Box<Box<dyn ErasedAdapterTransaction>> = Box::from_raw(transaction as *mut _);
}
