use ecap;
use ecap::common::Body;
use mopa::Any;

use common;

use host::Host as ErasedHost;

pub trait Message: Any {
    fn print_self(&self) {
        unsafe {
            println!("dyn message is {}", ::std::intrinsics::type_name::<Self>());
        }
    }

    fn clone(&self) -> Box<dyn Message>;

    ///// Always present, determines direction
    /////
    ///// XXX: We cannot do FirstLine without additional code to subclass into the other traits
    ///// XXX: Should this return an enum?
    //fn first_line_mut(&mut self) -> &mut Self::FirstLine;
    //fn first_line(&self) -> &Self::FirstLine;

    //fn header_mut(&mut self) -> &mut Self::Header;
    //fn header(&self) -> &Self::Header;

    //fn add_body(&mut self);
    //fn body_mut(&mut self) -> Option<&mut Self::Body>;
    fn body<'a>(&'a self) -> Option<&'a (dyn Body + 'static)>;

    //fn add_trailer(&mut self); // XXX: throws by default
    //fn trailer_mut(&mut self) -> &mut Self::Trailer;
    //fn trailer(&self) -> &Self::Trailer;
}

//impl Message {
//    fn downcast_ref<T: Any>(&self) -> Option<&T> {
//        self.downcast_ref::<T>()
//    }
//}

mopafy!(Message);

impl<U, H, T, FL, MC> Message for U
where
    U: ecap::common::Message<
        dyn ErasedHost,
        Header = H,
        Trailer = T,
        FirstLine = FL,
        MessageClone = MC,
    >,
    U: 'static,
    MC: ecap::common::Message<dyn ErasedHost> + 'static,
    FL: ecap::common::header::FirstLine + ?Sized,
    H: ecap::common::header::Header + ?Sized,
    T: ecap::common::header::Header + ?Sized,
{
    fn clone(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }

    fn body<'a>(&'a self) -> Option<&'a (dyn Body + 'static)> {
        match self.body() {
            Some(body) => Some(body),
            None => None,
        }
    }
}

impl ecap::common::Message<dyn ErasedHost> for dyn Message {
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
    fn body_mut(&mut self) -> Option<&mut (dyn Body + 'static)> {
        Self::body_mut(self)
    }
    fn body(&self) -> Option<&(dyn Body + 'static)> {
        <Self as Message>::body(self)
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
