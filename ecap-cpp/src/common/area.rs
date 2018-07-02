use ecap::common::area::{Area, Details, DetailsConstructor, DetailsStack};
use ffi;
use libc::c_char;
use std::mem;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::slice;

pub struct CppArea(ffi::Area);

impl CppArea {
    pub fn from_raw(raw: ffi::Area) -> CppArea {
        CppArea(raw)
    }

    pub fn from_bytes(v: &[u8]) -> CppArea {
        unsafe {
            CppArea::from_raw(ffi::rust_area_new_slice(
                v.as_ptr() as *const c_char,
                v.len(),
            ))
        }
    }

    pub fn from_area(area: Area) -> CppArea {
        // FIXME: avoid this copy
        CppArea::from_bytes(area.as_bytes())
    }

    pub fn into_raw(self) -> ffi::Area {
        unsafe {
            let value: ffi::Area = ptr::read(&self.0);
            mem::forget(self);
            value
        }
    }

    pub fn as_ptr(&self) -> *const ffi::Area {
        &self.0
    }

    pub fn as_ptr_mut(&mut self) -> *mut ffi::Area {
        &mut self.0
    }
}

impl DetailsConstructor for CppArea {
    fn details(self) -> DetailsStack {
        Rc::new(self).details()
    }
}

impl AsRef<[u8]> for CppArea {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.buf as *const u8, self.0.size) }
    }
}

impl Drop for CppArea {
    fn drop(&mut self) {
        unsafe {
            ffi::rust_area_free(&mut self.0);
        }
    }
}
