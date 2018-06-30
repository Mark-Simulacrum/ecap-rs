use ffi;
use libc::{c_char, c_void, size_t};

use ecap::common::{Area, Name, NamedValueVisitor, Options};

foreign_ref!(pub struct CppOptions(ffi::Options));

impl Options for CppOptions {
    fn option(&self, name: &Name) -> Option<&Area> {
        unimplemented!()
        //let name_s = name.image().unwrap_or("".into());
        //unsafe {
        //    let area = ffi::options_option(
        //        self as *const _ as *const _,
        //        name_s.as_ptr() as *const _,
        //        name_s.len(),
        //    );
        //    Some(Area::from_bytes(::std::slice::from_raw_parts(
        //        area.buf as *mut u8 as *const u8,
        //        area.size,
        //    )))
        //}
    }

    fn visit_each<V: NamedValueVisitor>(&self, mut visitor: V) {
        let visitor_ptr = &mut visitor;
        unsafe {
            ffi::options_visit(
                self as *const _ as *const _,
                visitor_callback,
                visitor_ptr as *mut _ as *const c_void,
            );
        }
    }
}

extern "C" fn visitor_callback(
    name: *const ffi::Name,
    buf: *const c_char,
    len: size_t,
    cb: *const c_void,
) {
    assert!(!name.is_null());
    assert!(!buf.is_null());
    assert!(!cb.is_null());
    unsafe {
        let value = ::std::slice::from_raw_parts(buf as *const u8, len);
        let visitor = &mut **(cb as *mut *mut dyn NamedValueVisitor);
        let name = ffi::rust_name_image(name);
        let slice = ::std::slice::from_raw_parts(name.buf as *const u8, name.size);
        // FIXME: avoid utf8/copy
        let name = Name::new_known(String::from_utf8(slice.into()).unwrap());
        visitor.visit(&name, &Area::from_bytes(value));
    }
}
