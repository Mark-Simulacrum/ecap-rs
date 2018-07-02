use ffi;
use libc::{c_char, c_int, c_void, size_t};

use common::CppName;
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
                self.as_ptr(),
                visitor_callback,
                visitor_ptr as *mut _ as *const c_void,
            );
        }
    }
}

extern "C" fn visitor_callback(
    name: ffi::Name,
    buf: *const c_char,
    len: size_t,
    cb: *const c_void,
) {
    assert!(!buf.is_null());
    assert!(!cb.is_null());
    unsafe {
        let value = ::std::slice::from_raw_parts(buf as *const u8, len);
        let visitor = &mut **(cb as *mut *mut dyn NamedValueVisitor);

        let name = CppName::from_raw(&name);
        visitor.visit(&name, &Area::from_bytes(value));
    }
}
