pub mod common;
pub mod host;
pub mod adapter;

use adapter::{Service, Transaction};

// XXX: Naming, feature-completness
pub struct AllocatedTransaction<'a>(pub Box<dyn Transaction + 'a>);

impl<'a> AllocatedTransaction<'a> {
    pub fn new<T: Transaction + 'a>(transaction: T) -> Self {
        AllocatedTransaction(Box::new(transaction))
    }
}

pub trait Translator: Send + Sync {
    fn register_service(&self, service: Box<dyn Service + Send + Sync>);
}
