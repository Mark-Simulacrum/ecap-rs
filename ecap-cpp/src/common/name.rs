use ecap::common::name::{Id, Name};
use ffi;
use libc::{c_char, c_int};
use std::marker::PhantomData;
use std::{mem, ptr, slice, str};

pub struct CppName<'a: 'b, 'b> {
    cpp: ffi::Name,
    name: PhantomData<&'b Name<'a>>,
}

impl<'a, 'b> CppName<'a, 'b> {
    pub fn from_name(name: &'a Name<'b>) -> CppName<'a, 'b> {
        unsafe {
            let cpp_name = ffi::Name {
                image: ffi::PStr {
                    size: name.image().map(|i| i.len()).unwrap_or(0),
                    buf: name.image()
                        .map(|i| i.as_ptr() as *const c_char)
                        .unwrap_or(ptr::null()),
                },
                host_id: name.host_id()
                    .map_or(c_int::min_value(), |hid| hid as c_int),
                id: match name.id() {
                    Id::Unknown => 0,
                    Id::Unidentified => 1,
                    Id::Id(id) => id as i32,
                },
                phantom: PhantomData,
            };

            CppName {
                cpp: cpp_name,
                name: PhantomData,
            }
        }
    }

    pub fn from_raw(name: &'a ffi::Name) -> Name<'a> {
        unsafe {
            let image = slice::from_raw_parts(name.image.buf as *const u8, name.image.size);
            let image = str::from_utf8(image).unwrap(); // XXX: expensive, also, should this be here?
            let id = match name.id {
                0 => Id::Unknown,
                1 => Id::Unidentified,
                other => Id::Id(other as u32),
            };
            let host_id = if name.host_id == c_int::min_value() {
                None
            } else {
                Some(name.host_id as u32)
            };
            Name::from_raw(image, id, host_id)
        }
    }

    pub fn as_ptr(&self) -> *const ffi::Name {
        &self.cpp
    }

    // XXX: probably don't want this
    pub fn as_ptr_mut(&mut self) -> *mut ffi::Name {
        &mut self.cpp
    }
}
