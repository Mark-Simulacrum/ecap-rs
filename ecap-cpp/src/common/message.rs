use ffi;
use libc::c_char;
use std::ops;

use common::body::CppBody;
use common::{CppArea, CppName};
use ecap::common::header::{FirstLine, Header};
use ecap::common::{Area, Body, Message as ConcreteMessage, Name, NamedValueVisitor, Version};
use host::CppHost;

use erased_ecap::common::header::FirstLine as ErasedFirstLine;
use erased_ecap::common::header::Header as ErasedHeader;

use erased_ecap::host::Host as ErasedHost;

foreign_ref!(pub struct CppMessage(ffi::Message));

foreign_ref!(pub struct CppHeader(ffi::Header));

impl Header for CppHeader {
    fn contains_field(&self, field: &Name) -> bool {
        unimplemented!()
    }

    fn get(&self, field: &Name) -> Option<&Area> {
        unimplemented!()
    }

    fn insert(&mut self, field: Name, value: Area) {
        unsafe {
            let field = CppName::from_name(&field);
            let value = CppArea::from_area(value);
            ffi::rust_shim_header_add(self.as_ptr_mut(), field.as_ptr(), value.as_ptr());
        }
    }

    fn remove_any(&mut self, field: &Name) {
        unsafe {
            let field = CppName::from_name(field);
            ffi::rust_shim_header_remove_any(self.as_ptr_mut(), field.as_ptr());
        }
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
    fn set_version(&mut self, _version: Version) {
        unimplemented!()
    }

    fn protocol(&self) -> &Name {
        unimplemented!()
    }
    fn set_protocol(&mut self, _protocol: Name) {
        unimplemented!()
    }
}

impl ConcreteMessage<CppHost> for CppMessage {
    type MessageClone = SharedPtrMessage;

    fn clone(&self) -> Self::MessageClone {
        unsafe { SharedPtrMessage(ffi::rust_shim_message_clone(self.as_ptr())) }
    }
    fn first_line_mut(&mut self) -> &mut CppFirstLine {
        unimplemented!()
    }
    fn first_line(&self) -> &CppFirstLine {
        unimplemented!()
    }
    fn header_mut(&mut self) -> &mut CppHeader {
        unsafe { CppHeader::from_ptr_mut(ffi::rust_shim_message_header_mut(self.as_ptr_mut())) }
    }
    fn header(&self) -> &CppHeader {
        unimplemented!()
    }
    fn add_body(&mut self) {
        unimplemented!()
    }
    fn body_mut(&mut self) -> Option<&mut CppBody> {
        unimplemented!()
    }
    fn body(&self) -> Option<&CppBody> {
        unsafe {
            let ptr = ffi::rust_shim_message_body(self.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CppBody::from_ptr(ptr))
            }
        }
    }
    fn add_trailer(&mut self) {
        unimplemented!()
    }
    fn trailer_mut(&mut self) -> &mut CppTrailer {
        unimplemented!()
    }
    fn trailer(&self) -> &CppTrailer {
        unimplemented!()
    }
}

impl ConcreteMessage<dyn ErasedHost> for CppMessage {
    type MessageClone = SharedPtrMessage;

    fn clone(&self) -> Self::MessageClone {
        <Self as ConcreteMessage<CppHost>>::clone(self)
    }
    fn first_line_mut(&mut self) -> &mut (dyn ErasedFirstLine + 'static) {
        unimplemented!()
    }
    fn first_line(&self) -> &(dyn ErasedFirstLine + 'static) {
        unimplemented!()
    }
    fn header_mut(&mut self) -> &mut (dyn ErasedHeader + 'static) {
        <Self as ConcreteMessage<CppHost>>::header_mut(self)
    }
    fn header(&self) -> &(dyn ErasedHeader + 'static) {
        unimplemented!()
    }
    fn add_body(&mut self) {
        unimplemented!()
    }
    fn body_mut(&mut self) -> Option<&mut (dyn Body + 'static)> {
        unimplemented!()
    }
    fn body(&self) -> Option<&(dyn Body + 'static)> {
        match <Self as ConcreteMessage<CppHost>>::body(self) {
            Some(body) => Some(body),
            None => None,
        }
    }
    fn add_trailer(&mut self) {
        unimplemented!()
    }
    fn trailer_mut(&mut self) -> &mut (dyn ErasedHeader + 'static) {
        unimplemented!()
    }
    fn trailer(&self) -> &(dyn ErasedHeader + 'static) {
        unimplemented!()
    }
}

pub struct SharedPtrMessage(pub ffi::SharedPtrMessage);

impl SharedPtrMessage {
    pub fn as_ptr(&self) -> *const ffi::SharedPtrMessage {
        &self.0
    }
}

impl ops::Deref for SharedPtrMessage {
    type Target = CppMessage;
    fn deref(&self) -> &CppMessage {
        unsafe {
            let msg = ffi::rust_shim_shared_ptr_message_ref(&self.0);

            CppMessage::from_ptr(msg)
        }
    }
}

impl ops::DerefMut for SharedPtrMessage {
    fn deref_mut(&mut self) -> &mut CppMessage {
        unsafe {
            let msg = ffi::rust_shim_shared_ptr_message_ref_mut(&mut self.0);
            CppMessage::from_ptr_mut(msg)
        }
    }
}

impl Drop for SharedPtrMessage {
    fn drop(&mut self) {
        unsafe {
            ffi::rust_shim_shared_ptr_message_free(&mut self.0);
        }
    }
}

impl ConcreteMessage<CppHost> for SharedPtrMessage {
    type MessageClone = SharedPtrMessage;

    fn clone(&self) -> Self::MessageClone {
        unimplemented!()
    }
    fn first_line_mut(&mut self) -> &mut CppFirstLine {
        unimplemented!()
    }
    fn first_line(&self) -> &CppFirstLine {
        unimplemented!()
    }
    fn header_mut(&mut self) -> &mut CppHeader {
        unimplemented!()
    }
    fn header(&self) -> &CppHeader {
        unimplemented!()
    }
    fn add_body(&mut self) {
        unimplemented!()
    }
    fn body_mut(&mut self) -> Option<&mut CppBody> {
        unimplemented!()
    }
    fn body(&self) -> Option<&CppBody> {
        unimplemented!()
    }
    fn add_trailer(&mut self) {
        unimplemented!()
    }
    fn trailer_mut(&mut self) -> &mut CppTrailer {
        unimplemented!()
    }
    fn trailer(&self) -> &CppTrailer {
        unimplemented!()
    }
}

impl ConcreteMessage<dyn ErasedHost> for SharedPtrMessage {
    type MessageClone = SharedPtrMessage;

    fn clone(&self) -> Self::MessageClone {
        unimplemented!()
    }
    fn first_line_mut(&mut self) -> &mut (dyn ErasedFirstLine + 'static) {
        unimplemented!()
    }
    fn first_line(&self) -> &(dyn ErasedFirstLine + 'static) {
        unimplemented!()
    }
    fn header_mut(&mut self) -> &mut (dyn ErasedHeader + 'static) {
        let msg = <Self as ops::DerefMut>::deref_mut(self);
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::header_mut(msg)
    }
    fn header(&self) -> &(dyn ErasedHeader + 'static) {
        unimplemented!()
    }
    fn add_body(&mut self) {
        unimplemented!()
    }
    fn body_mut(&mut self) -> Option<&mut (dyn Body + 'static)> {
        unimplemented!()
    }
    fn body(&self) -> Option<&(dyn Body + 'static)> {
        let msg = <Self as ops::Deref>::deref(self);
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::body(msg)
    }
    fn add_trailer(&mut self) {
        unimplemented!()
    }
    fn trailer_mut(&mut self) -> &mut (dyn ErasedHeader + 'static) {
        unimplemented!()
    }
    fn trailer(&self) -> &(dyn ErasedHeader + 'static) {
        unimplemented!()
    }
}
