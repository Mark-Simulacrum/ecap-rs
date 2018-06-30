use common::header::{FirstLine, Header};
use common::Body;

pub trait Message {
    type Header: Header + ?Sized;
    type Trailer: Header + ?Sized;
    type FirstLine: FirstLine + ?Sized;
    type Body: Body + ?Sized;

    // FIXME: Message bound here may be too limiting
    type MessageClone: Message;

    fn clone(&self) -> Self::MessageClone;

    /// Always present, determines direction
    ///
    /// XXX: We cannot do FirstLine without additional code to subclass into the other traits
    /// XXX: Should this return an enum?
    fn first_line_mut(&mut self) -> &mut Self::FirstLine;
    fn first_line(&self) -> &Self::FirstLine;

    fn header_mut(&mut self) -> &mut Self::Header;
    fn header(&self) -> &Self::Header;

    fn add_body(&mut self);
    fn body_mut(&mut self) -> &mut Self::Body;
    fn body(&self) -> &Self::Body;

    fn add_trailer(&mut self); // XXX: throws by default
    fn trailer_mut(&mut self) -> &mut Self::Trailer;
    fn trailer(&self) -> &Self::Trailer;
}

impl<T: Message + ?Sized> Message for Box<T> {
    type Header = T::Header;
    type Trailer = T::Trailer;
    type FirstLine = T::FirstLine;
    type Body = T::Body;
    type MessageClone = T::MessageClone;

    fn clone(&self) -> Self::MessageClone {
        (&**self).clone()
    }

    fn first_line_mut(&mut self) -> &mut Self::FirstLine {
        (&mut **self).first_line_mut()
    }
    fn first_line(&self) -> &Self::FirstLine {
        (&**self).first_line()
    }

    fn header_mut(&mut self) -> &mut Self::Header {
        (&mut **self).header_mut()
    }
    fn header(&self) -> &Self::Header {
        (&**self).header()
    }

    fn add_body(&mut self) {
        (&mut **self).add_body();
    }
    fn body_mut(&mut self) -> &mut Self::Body {
        (&mut **self).body_mut()
    }
    fn body(&self) -> &Self::Body {
        (&**self).body()
    }

    fn add_trailer(&mut self) {
        (&mut **self).add_trailer();
    }
    fn trailer_mut(&mut self) -> &mut Self::Trailer {
        (&mut **self).trailer_mut()
    }
    fn trailer(&self) -> &Self::Trailer {
        (&**self).trailer()
    }
}
