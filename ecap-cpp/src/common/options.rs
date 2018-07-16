use ffi;
use libc::{c_char, c_int, c_void, size_t};

use common::{CppArea, CppName};
use ecap::common::{Area, Name, NamedValueVisitor, Options};

use call_ffi_maybe_panic;

foreign_ref!(pub struct CppOptions(ffi::Options));

impl Options for CppOptions {
    fn option(&self, name: &Name) -> Option<Area> {
        let name = CppName::from_name(name);
        unsafe {
            let area = call_ffi_maybe_panic(|raw| unsafe {
                ffi::options_option(self.as_ptr(), name.as_ptr(), raw)
            });
            Some(CppArea::from_raw(area).into())
        }
    }

    fn visit_each<V: NamedValueVisitor>(&self, mut visitor: V) {
        let visitor_ptr = &mut visitor;
        call_ffi_maybe_panic(|_: *mut ()| unsafe {
            ffi::options_visit(
                self.as_ptr(),
                visitor_callback,
                visitor_ptr as *mut _ as *mut c_void,
            )
        });
    }
}

// XXX: Must handle panics/exceptions
pub extern "C" fn visitor_callback(name: ffi::Name, area: ffi::Area, cb: *mut c_void) {
    assert!(!cb.is_null());
    unsafe {
        let visitor = &mut **(cb as *mut *mut dyn NamedValueVisitor);

        let name = CppName::from_raw(&name);
        visitor.visit(&name, &Area::new(CppArea::from_raw(area)));
    }
}
