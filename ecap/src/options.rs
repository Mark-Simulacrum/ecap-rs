use {Area, ffi};
use libc::c_void;

foreign_ref!(pub struct Options(ffi::Options));

impl Options {
    pub fn option(&self, name: &[u8]) -> Area {
        unsafe {
            Area::from_raw(ffi::options_option(self.as_ptr(), name.as_ptr() as *const _, name.len()))
        }
    }

    pub fn visit(&self, callback: fn(&ffi::Name, &[u8])) {
        unsafe {
            ffi::options_visit(self.as_ptr(), ffi::visitor_callback, callback as *const c_void);
        }
    }
}
