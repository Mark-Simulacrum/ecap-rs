use ffi;
use std::sync::Arc;

use ecap::common::header::{FirstLine, Header};
use ecap::common::Body;
use ecap::common::Message;

// XXX: this is the wrong type?
pub type SharedPtrMessage = Arc<dyn Message>;

foreign_ref!(pub struct CppMessage(ffi::Message));

impl Message for CppMessage {
    fn clone(&self) -> Arc<dyn Message> {
        unimplemented!()
    }
    fn first_line_mut(&mut self) -> &mut dyn FirstLine {
        unimplemented!()
    }
    fn first_line(&self) -> &dyn FirstLine {
        unimplemented!()
    }
    fn header_mut(&mut self) -> &dyn Header {
        unimplemented!()
    }
    fn header(&self) -> &dyn Header {
        unimplemented!()
    }
    fn add_body(&mut self) {
        unimplemented!()
    }
    fn body_mut(&mut self) -> &mut dyn Body {
        unimplemented!()
    }
    fn body(&self) -> &dyn Body {
        unimplemented!()
    }
    fn add_trailer(&mut self) {
        unimplemented!()
    }
    fn trailer_mut(&mut self) -> &mut dyn Header {
        unimplemented!()
    }
    fn trailer(&self) -> &dyn Header {
        unimplemented!()
    }
}
