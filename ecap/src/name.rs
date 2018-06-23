use ffi;
use libc::{c_char, c_int};
use std::{fmt, slice};

pub struct Name(ffi::Name);

impl Name {
    pub fn as_ptr(&self) -> *const ffi::Name {
        &self.0 as *const _
    }

    pub fn as_ptr_mut(&mut self) -> *mut ffi::Name {
        &mut self.0 as *mut _
    }

    pub fn unknown() -> Name {
        unsafe { Name(ffi::rust_name_new_unknown()) }
    }

    pub fn with_image(image: &[u8]) -> Name {
        unsafe {
            Name(ffi::rust_name_new_image(
                image.as_ptr() as *const c_char,
                image.len(),
            ))
        }
    }

    pub fn with_image_and_identified(image: &[u8], id: c_int) -> Name {
        unsafe {
            Name(ffi::rust_name_new_image_id(
                image.as_ptr() as *const c_char,
                image.len(),
                id,
            ))
        }
    }

    pub fn image<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let ffi::PStr { size, buf } = ffi::rust_name_image(&self.0);
            slice::from_raw_parts(buf as *const u8, size)
        }
    }

    pub fn known(&self) -> bool {
        unsafe { ffi::rust_name_known(&self.0) }
    }

    pub fn identified(&self) -> bool {
        unsafe { ffi::rust_name_identified(&self.0) }
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Name")
            .field("identified", &self.identified())
            .field("known", &self.known())
            .field("image", &self.image())
            .finish()
    }
}

#[test]
fn name_eq_1() {
    assert_eq!(Name::with_image(b"apple"), Name::with_image(b"apple"));
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::rust_name_eq(self.as_ptr(), other.as_ptr()) }
    }
}

impl Drop for Name {
    fn drop(&mut self) {
        unsafe {
            ffi::rust_name_free(self.as_ptr_mut());
        }
    }
}
