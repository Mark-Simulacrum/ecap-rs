pub mod adapter;
pub mod common;
pub mod host;

use adapter::Service;
use host::Host;

pub trait Translator: Send + Sync {
    fn register_service<H: ?Sized + Host, T: Service<H>>(&self, service: T);
}
