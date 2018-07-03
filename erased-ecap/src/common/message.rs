use ecap;
use ecap::common::Body;
use mopa::Any;

use common::header::{FirstLine, Header};

use host::Host as ErasedHost;

pub trait Message: Any {
    fn clone(&self) -> Box<dyn Message>;

    fn first_line_mut<'a>(&'a mut self) -> &'a mut (dyn FirstLine + 'static);
    fn first_line<'a>(&'a self) -> &'a (dyn FirstLine + 'static);

    fn header_mut<'a>(&'a mut self) -> &'a mut (dyn Header + 'static);
    fn header<'a>(&'a self) -> &'a (dyn Header + 'static);

    fn add_body(&mut self);
    fn body_mut<'a>(&'a mut self) -> Option<&'a mut (dyn Body + 'static)>;
    fn body<'a>(&'a self) -> Option<&'a (dyn Body + 'static)>;

    fn add_trailer(&mut self); // XXX: throws by default
    fn trailer_mut<'a>(&'a mut self) -> &'a mut (dyn Header + 'static);
    fn trailer<'a>(&'a self) -> &'a (dyn Header + 'static);
}

mopafy!(Message);

impl<U, MC> Message for U
where
    U: ecap::common::Message<dyn ErasedHost, MessageClone = MC> + 'static,
    MC: ecap::common::Message<dyn ErasedHost> + 'static,
{
    fn clone(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }

    fn first_line_mut<'a>(&'a mut self) -> &'a mut (dyn FirstLine + 'static) {
        self.first_line_mut()
    }

    fn first_line<'a>(&'a self) -> &'a (dyn FirstLine + 'static) {
        self.first_line()
    }

    fn header_mut<'a>(&'a mut self) -> &'a mut (dyn Header + 'static) {
        self.header_mut()
    }

    fn header<'a>(&'a self) -> &'a (dyn Header + 'static) {
        self.header()
    }

    fn add_body<'a>(&'a mut self) {
        self.add_body()
    }

    fn body_mut<'a>(&'a mut self) -> Option<&'a mut (dyn Body + 'static)> {
        match self.body_mut() {
            Some(body) => Some(body),
            None => None,
        }
    }

    fn body<'a>(&'a self) -> Option<&'a (dyn Body + 'static)> {
        match self.body() {
            Some(body) => Some(body),
            None => None,
        }
    }

    fn add_trailer(&mut self) {
        self.add_trailer()
    }

    fn trailer_mut<'a>(&'a mut self) -> &'a mut (dyn Header + 'static) {
        self.trailer_mut()
    }

    fn trailer<'a>(&'a self) -> &'a (dyn Header + 'static) {
        self.trailer()
    }
}

impl ecap::common::Message<dyn ErasedHost> for dyn Message {
    type MessageClone = Box<dyn Message>;

    fn clone(&self) -> Self::MessageClone {
        Self::clone(self)
    }

    fn first_line_mut(&mut self) -> &mut (dyn FirstLine + 'static) {
        Self::first_line_mut(self)
    }
    fn first_line(&self) -> &(dyn FirstLine + 'static) {
        Self::first_line(self)
    }

    fn header_mut(&mut self) -> &mut (dyn Header + 'static) {
        <Self as Message>::header_mut(self)
    }
    fn header(&self) -> &(dyn Header + 'static) {
        <Self as Message>::header(self)
    }

    fn add_body(&mut self) {
        Self::add_body(self)
    }
    fn body_mut(&mut self) -> Option<&mut (dyn Body + 'static)> {
        Self::body_mut(self)
    }
    fn body(&self) -> Option<&(dyn Body + 'static)> {
        <Self as Message>::body(self)
    }

    fn add_trailer(&mut self) {
        Self::add_trailer(self)
    }
    fn trailer_mut(&mut self) -> &mut (dyn Header + 'static) {
        Self::trailer_mut(self)
    }
    fn trailer(&self) -> &(dyn Header + 'static) {
        Self::trailer(self)
    }
}
