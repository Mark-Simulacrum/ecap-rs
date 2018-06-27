/// Message body buffer, shared by producer and consumer.
///
/// Usually implemented in hosts.
///
/// This trait may be deprecated in favor of moving this information
/// into Message itself in the future.
pub trait Body {
    /// Returns the size of the Body.
    ///
    /// Will return `None` if the size is not known.
    fn size(&self) -> Option<u64>;
}
