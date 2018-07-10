pub mod adapter;
pub mod common;
pub mod host;

use adapter::Service;
use host::Host;

pub trait Translator: Send + Sync {
    fn register_service<H, T>(&self, service: T)
    where
        H: Host + ?Sized,
        T: Service<H> + 'static;
}
