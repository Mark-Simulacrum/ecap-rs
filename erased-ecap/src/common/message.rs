use ecap;
use mopa::Any;

use common;

pub trait Message: Any {
    // nothing
}

mopafy!(Message);

impl<U, H, T, FL, B, MC> Message for U
where
    U: ecap::common::Message<Header = H, Trailer = T, FirstLine = FL, Body = B, MessageClone = MC>,
    U: 'static,
    MC: ecap::common::Message,
    B: ecap::common::Body + ?Sized,
    FL: ecap::common::header::FirstLine + ?Sized,
    H: ecap::common::header::Header + ?Sized,
    T: ecap::common::header::Header + ?Sized,
{
}

impl ecap::common::Message for dyn Message {
    type Body = dyn ecap::common::Body;
    type MessageClone = Box<dyn Message>;
    type FirstLine = dyn ecap::common::header::FirstLine;
    type Header = dyn common::header::Header;
    type Trailer = dyn common::header::Header;

    fn clone(&self) -> Self::MessageClone {
        Self::clone(self)
    }

    fn first_line_mut(&mut self) -> &mut Self::FirstLine {
        Self::first_line_mut(self)
    }
    fn first_line(&self) -> &Self::FirstLine {
        Self::first_line(self)
    }

    fn header_mut(&mut self) -> &mut Self::Header {
        Self::header_mut(self)
    }
    fn header(&self) -> &Self::Header {
        Self::header(self)
    }

    fn add_body(&mut self) {
        Self::add_body(self)
    }
    fn body_mut(&mut self) -> &mut Self::Body {
        Self::body_mut(self)
    }
    fn body(&self) -> &Self::Body {
        Self::body(self)
    }

    fn add_trailer(&mut self) {
        Self::add_trailer(self)
    }
    fn trailer_mut(&mut self) -> &mut Self::Trailer {
        Self::trailer_mut(self)
    }
    fn trailer(&self) -> &Self::Trailer {
        Self::trailer(self)
    }
}
