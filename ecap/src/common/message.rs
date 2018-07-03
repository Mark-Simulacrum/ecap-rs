use host::Host;

pub trait Message<H: ?Sized + Host> {
    // FIXME: Message bound here may be too limiting
    type MessageClone: Message<H> + 'static;

    fn clone(&self) -> Self::MessageClone;

    /// Always present, determines direction
    ///
    /// XXX: We cannot do FirstLine without additional code to subclass into the other traits
    /// XXX: Should this return an enum?
    fn first_line_mut(&mut self) -> &mut H::FirstLine;
    fn first_line(&self) -> &H::FirstLine;

    fn header_mut(&mut self) -> &mut H::Header;
    fn header(&self) -> &H::Header;

    fn add_body(&mut self);
    fn body_mut(&mut self) -> Option<&mut H::Body>;
    fn body(&self) -> Option<&H::Body>;

    fn add_trailer(&mut self); // XXX: throws by default
    fn trailer_mut(&mut self) -> &mut H::Trailer;
    fn trailer(&self) -> &H::Trailer;
}

impl<H: Host + ?Sized, T: Message<H> + ?Sized> Message<H> for Box<T> {
    type MessageClone = T::MessageClone;

    fn clone(&self) -> Self::MessageClone {
        (&**self).clone()
    }

    fn first_line_mut(&mut self) -> &mut H::FirstLine {
        (&mut **self).first_line_mut()
    }
    fn first_line(&self) -> &H::FirstLine {
        (&**self).first_line()
    }

    fn header_mut(&mut self) -> &mut H::Header {
        (&mut **self).header_mut()
    }
    fn header(&self) -> &H::Header {
        (&**self).header()
    }

    fn add_body(&mut self) {
        (&mut **self).add_body();
    }
    fn body_mut(&mut self) -> Option<&mut H::Body> {
        (&mut **self).body_mut()
    }
    fn body(&self) -> Option<&H::Body> {
        (&**self).body()
    }

    fn add_trailer(&mut self) {
        (&mut **self).add_trailer();
    }
    fn trailer_mut(&mut self) -> &mut H::Trailer {
        (&mut **self).trailer_mut()
    }
    fn trailer(&self) -> &H::Trailer {
        (&**self).trailer()
    }
}
