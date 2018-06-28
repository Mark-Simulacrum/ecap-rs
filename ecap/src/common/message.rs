use std::sync::Arc;

use common::Body;
use common::header::{Header, FirstLine};

pub trait Message {
    // FIXME: Return type here is too limiting?
    fn clone(&self) -> Arc<dyn Message>;

    /// Always present, determines direction
    ///
    /// XXX: We cannot do FirstLine without additional code to subclass into the other traits
    /// XXX: Should this return an enum?
    fn first_line_mut(&mut self) -> &mut dyn FirstLine;
    fn first_line(&self) -> &dyn FirstLine;

    fn header_mut(&mut self) -> &dyn Header;
    fn header(&self) -> &dyn Header;

    fn add_body(&mut self);
    fn body_mut(&mut self) -> &mut dyn Body;
    fn body(&self) -> &dyn Body;

    fn add_trailer(&mut self); // XXX: throws by default
    fn trailer_mut(&mut self) -> &mut dyn Header;
    fn trailer(&self) -> &dyn Header;
}
