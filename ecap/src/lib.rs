#[macro_use]
extern crate mopa;
#[macro_use]
extern crate parse_generics_shim;

pub mod adapter;
pub mod common;
pub mod host;

//// XXX: Naming, feature-completness
//pub struct AllocatedTransaction<'a>(pub Box<dyn Transaction + 'a>);
//
//impl<'a> AllocatedTransaction<'a> {
//    pub fn new<T: Transaction + 'a>(transaction: T) -> Self {
//        AllocatedTransaction(Box::new(transaction))
//    }
//}

use adapter::Service;
use host::Host;

pub trait Translator: Send + Sync {
    fn register_service<H: ?Sized + Host, T: Service<H>>(&self, service: T);
}
