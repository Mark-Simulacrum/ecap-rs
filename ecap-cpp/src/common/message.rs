use ffi;
use libc::{c_char, c_void};
use std::ops;

use common::body::CppBody;
use common::{options, CppArea, CppName, CppVersion};
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
        unsafe {
            let field = CppName::from_name(field);
            ffi::rust_shim_header_has_any(self.as_ptr(), field.as_ptr())
        }
    }

    fn get(&self, field: &Name) -> Option<Area> {
        unsafe {
            let field = CppName::from_name(field);
            let cpp_area =
                CppArea::from_raw(ffi::rust_shim_header_value(self.as_ptr(), field.as_ptr()));
            let area: Area = cpp_area.into();
            if area.as_bytes().is_empty() {
                None
            } else {
                Some(area)
            }
        }
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

    fn visit_each<V: NamedValueVisitor>(&self, mut visitor: &mut V) {
        let visitor = &mut visitor;
        unsafe {
            ffi::rust_shim_header_visit_each(
                self.as_ptr(),
                options::visitor_callback,
                visitor as *mut _ as *mut c_void,
            )
        }
    }

    fn image(&self) -> Area {
        unsafe { CppArea::from_raw(ffi::rust_shim_header_image(self.as_ptr())).into() }
    }

    fn parse(&mut self, buf: &Area) -> Result<(), ()> {
        let cpp_area = CppArea::from_area(buf.clone());
        // XXX: This throws exceptions
        unsafe {
            ffi::rust_shim_header_parse(self.as_ptr_mut(), cpp_area.as_ptr());
        }
        Ok(())
    }
}

foreign_ref!(pub struct CppFirstLine(ffi::FirstLine));

impl FirstLine for CppFirstLine {
    fn version(&self) -> Version {
        unsafe { CppVersion::from_raw(ffi::rust_shim_first_line_version(self.as_ptr())) }
    }
    fn set_version(&mut self, version: Version) {
        let v = CppVersion::to_raw(version);
        unsafe { ffi::rust_shim_first_line_set_version(self.as_ptr_mut(), &v) }
    }

    fn protocol(&self) -> Name {
        unsafe {
            let raw = ffi::rust_shim_first_line_protocol(self.as_ptr());
            // XXX: copy!
            CppName::from_raw(&raw).to_owned()
        }
    }
    fn set_protocol(&mut self, protocol: Name) {
        unsafe {
            let raw = CppName::from_name(&protocol);
            ffi::rust_shim_first_line_set_protocol(self.as_ptr_mut(), raw.as_ptr());
        }
    }
}

impl ConcreteMessage<CppHost> for CppMessage {
    type MessageClone = SharedPtrMessage;

    fn clone(&self) -> Self::MessageClone {
        unsafe { SharedPtrMessage(ffi::rust_shim_message_clone(self.as_ptr())) }
    }
    fn first_line_mut(&mut self) -> &mut CppFirstLine {
        unsafe {
            CppFirstLine::from_ptr_mut(ffi::rust_shim_message_first_line_mut(self.as_ptr_mut()))
        }
    }
    fn first_line(&self) -> &CppFirstLine {
        unsafe { CppFirstLine::from_ptr(ffi::rust_shim_message_first_line(self.as_ptr())) }
    }
    fn header_mut(&mut self) -> &mut CppHeader {
        unsafe { CppHeader::from_ptr_mut(ffi::rust_shim_message_header_mut(self.as_ptr_mut())) }
    }
    fn header(&self) -> &CppHeader {
        unsafe { CppHeader::from_ptr(ffi::rust_shim_message_header(self.as_ptr())) }
    }
    fn add_body(&mut self) {
        unsafe { ffi::rust_shim_message_add_body(self.as_ptr_mut()) }
    }
    fn body_mut(&mut self) -> Option<&mut CppBody> {
        unsafe {
            let ptr = ffi::rust_shim_message_body_mut(self.as_ptr_mut());
            if ptr.is_null() {
                None
            } else {
                Some(CppBody::from_ptr_mut(ptr))
            }
        }
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
        unsafe {
            ffi::rust_shim_message_add_trailer(self.as_ptr_mut());
        }
    }
    fn trailer_mut(&mut self) -> &mut CppHeader {
        unsafe { CppHeader::from_ptr_mut(ffi::rust_shim_message_trailer_mut(self.as_ptr_mut())) }
    }
    fn trailer(&self) -> &CppHeader {
        unsafe { CppHeader::from_ptr(ffi::rust_shim_message_trailer(self.as_ptr())) }
    }
}

impl ConcreteMessage<dyn ErasedHost> for CppMessage {
    type MessageClone = SharedPtrMessage;

    fn clone(&self) -> Self::MessageClone {
        <Self as ConcreteMessage<CppHost>>::clone(self)
    }
    fn first_line_mut(&mut self) -> &mut (dyn ErasedFirstLine + 'static) {
        <Self as ConcreteMessage<CppHost>>::first_line_mut(self)
    }
    fn first_line(&self) -> &(dyn ErasedFirstLine + 'static) {
        <Self as ConcreteMessage<CppHost>>::first_line(self)
    }
    fn header_mut(&mut self) -> &mut (dyn ErasedHeader + 'static) {
        <Self as ConcreteMessage<CppHost>>::header_mut(self)
    }
    fn header(&self) -> &(dyn ErasedHeader + 'static) {
        <Self as ConcreteMessage<CppHost>>::header(self)
    }
    fn add_body(&mut self) {
        <Self as ConcreteMessage<CppHost>>::add_body(self)
    }
    fn body_mut(&mut self) -> Option<&mut (dyn Body + 'static)> {
        match <Self as ConcreteMessage<CppHost>>::body_mut(self) {
            Some(b) => Some(b),
            None => None,
        }
    }
    fn body(&self) -> Option<&(dyn Body + 'static)> {
        match <Self as ConcreteMessage<CppHost>>::body(self) {
            Some(body) => Some(body),
            None => None,
        }
    }
    fn add_trailer(&mut self) {
        <Self as ConcreteMessage<CppHost>>::add_trailer(self)
    }
    fn trailer_mut(&mut self) -> &mut (dyn ErasedHeader + 'static) {
        <Self as ConcreteMessage<CppHost>>::trailer_mut(self)
    }
    fn trailer(&self) -> &(dyn ErasedHeader + 'static) {
        <Self as ConcreteMessage<CppHost>>::trailer(self)
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
        <CppMessage as ConcreteMessage<CppHost>>::clone(self)
    }
    fn first_line_mut(&mut self) -> &mut CppFirstLine {
        <CppMessage as ConcreteMessage<CppHost>>::first_line_mut(self)
    }
    fn first_line(&self) -> &CppFirstLine {
        <CppMessage as ConcreteMessage<CppHost>>::first_line(self)
    }
    fn header_mut(&mut self) -> &mut CppHeader {
        <CppMessage as ConcreteMessage<CppHost>>::header_mut(self)
    }
    fn header(&self) -> &CppHeader {
        <CppMessage as ConcreteMessage<CppHost>>::header(self)
    }
    fn add_body(&mut self) {
        <CppMessage as ConcreteMessage<CppHost>>::add_body(self)
    }
    fn body_mut(&mut self) -> Option<&mut CppBody> {
        <CppMessage as ConcreteMessage<CppHost>>::body_mut(self)
    }
    fn body(&self) -> Option<&CppBody> {
        <CppMessage as ConcreteMessage<CppHost>>::body(self)
    }
    fn add_trailer(&mut self) {
        <CppMessage as ConcreteMessage<CppHost>>::add_trailer(self)
    }
    fn trailer_mut(&mut self) -> &mut CppHeader {
        <CppMessage as ConcreteMessage<CppHost>>::trailer_mut(self)
    }
    fn trailer(&self) -> &CppHeader {
        <CppMessage as ConcreteMessage<CppHost>>::trailer(self)
    }
}

impl ConcreteMessage<dyn ErasedHost> for SharedPtrMessage {
    type MessageClone = SharedPtrMessage;

    fn clone(&self) -> Self::MessageClone {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::clone(<Self as ops::Deref>::deref(self))
    }
    fn first_line_mut(&mut self) -> &mut (dyn ErasedFirstLine + 'static) {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::first_line_mut(
            <Self as ops::DerefMut>::deref_mut(self),
        )
    }
    fn first_line(&self) -> &(dyn ErasedFirstLine + 'static) {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::first_line(<Self as ops::Deref>::deref(
            self,
        ))
    }
    fn header_mut(&mut self) -> &mut (dyn ErasedHeader + 'static) {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::header_mut(
            <Self as ops::DerefMut>::deref_mut(self),
        )
    }
    fn header(&self) -> &(dyn ErasedHeader + 'static) {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::header(<Self as ops::Deref>::deref(self))
    }
    fn add_body(&mut self) {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::add_body(
            <Self as ops::DerefMut>::deref_mut(self),
        )
    }
    fn body_mut(&mut self) -> Option<&mut (dyn Body + 'static)> {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::body_mut(
            <Self as ops::DerefMut>::deref_mut(self),
        )
    }
    fn body(&self) -> Option<&(dyn Body + 'static)> {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::body(<Self as ops::Deref>::deref(self))
    }
    fn add_trailer(&mut self) {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::add_trailer(
            <Self as ops::DerefMut>::deref_mut(self),
        )
    }
    fn trailer_mut(&mut self) -> &mut (dyn ErasedHeader + 'static) {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::trailer_mut(
            <Self as ops::DerefMut>::deref_mut(self),
        )
    }
    fn trailer(&self) -> &(dyn ErasedHeader + 'static) {
        <CppMessage as ConcreteMessage<dyn ErasedHost>>::trailer(<Self as ops::Deref>::deref(self))
    }
}
