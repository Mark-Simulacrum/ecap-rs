use ffi;

use ecap::common::header::{FirstLine, Header};
use ecap::common::{Area, Body, Message, Name, NamedValueVisitor, Version};

// XXX: this is the wrong type?
pub type SharedPtrMessage = Box<CppMessage>;

foreign_ref!(pub struct CppMessage(ffi::Message));

pub struct CppBody;

impl Body for CppBody {
    fn size(&self) -> Option<u64> {
        unimplemented!()
    }
}

pub struct CppHeader;

impl Header for CppHeader {
    fn contains_field(&self, field: &Name) -> bool {
        unimplemented!()
    }

    fn get(&self, field: &Name) -> Option<&Area> {
        unimplemented!()
    }

    fn insert(&mut self, field: Name, value: Area) {
        unimplemented!()
    }

    fn remove_any(&mut self, field: &Name) {
        unimplemented!()
    }

    fn visit_each<V: NamedValueVisitor>(&self, visitor: &mut V) {
        unimplemented!()
    }

    fn image(&self) -> &Area {
        unimplemented!()
    }

    fn parse(&mut self, buf: &Area) -> Result<(), ()> {
        unimplemented!()
    }
}

pub struct CppTrailer;

impl Header for CppTrailer {
    fn contains_field(&self, field: &Name) -> bool {
        unimplemented!()
    }

    fn get(&self, field: &Name) -> Option<&Area> {
        unimplemented!()
    }

    fn insert(&mut self, field: Name, value: Area) {
        unimplemented!()
    }

    fn remove_any(&mut self, field: &Name) {
        unimplemented!()
    }

    fn visit_each<V: NamedValueVisitor>(&self, visitor: &mut V) {
        unimplemented!()
    }

    fn image(&self) -> &Area {
        unimplemented!()
    }

    fn parse(&mut self, buf: &Area) -> Result<(), ()> {
        unimplemented!()
    }
}

pub struct CppFirstLine;

impl FirstLine for CppFirstLine {
    fn version(&self) -> Version {
        unimplemented!()
    }
    fn set_version(&mut self, version: Version) {
        unimplemented!()
    }

    fn protocol(&self) -> &Name {
        unimplemented!()
    }
    fn set_protocol(&mut self, protocol: Name) {
        unimplemented!()
    }
}

impl Message for CppMessage {
    type Header = CppHeader;
    type Trailer = CppTrailer;
    type FirstLine = CppFirstLine;
    type Body = CppBody;
    type MessageClone = SharedPtrMessage;

    fn clone(&self) -> Self::MessageClone {
        unimplemented!()
    }
    fn first_line_mut(&mut self) -> &mut Self::FirstLine {
        unimplemented!()
    }
    fn first_line(&self) -> &Self::FirstLine {
        unimplemented!()
    }
    fn header_mut(&mut self) -> &mut Self::Header {
        unimplemented!()
    }
    fn header(&self) -> &Self::Header {
        unimplemented!()
    }
    fn add_body(&mut self) {
        unimplemented!()
    }
    fn body_mut(&mut self) -> &mut Self::Body {
        unimplemented!()
    }
    fn body(&self) -> &Self::Body {
        unimplemented!()
    }
    fn add_trailer(&mut self) {
        unimplemented!()
    }
    fn trailer_mut(&mut self) -> &mut Self::Trailer {
        unimplemented!()
    }
    fn trailer(&self) -> &Self::Trailer {
        unimplemented!()
    }
}
