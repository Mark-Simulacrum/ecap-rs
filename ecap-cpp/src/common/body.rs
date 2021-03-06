use call_ffi_maybe_panic;
use ffi;

use ecap::common::Body;

foreign_ref!(pub struct CppBody(ffi::Body));

impl Body for CppBody {
    fn size(&self) -> Option<u64> {
        unsafe {
            let size =
                call_ffi_maybe_panic(|raw| unsafe { ffi::rust_shim_body_size(self.as_ptr(), raw) });
            if size.known {
                Some(size.size)
            } else {
                None
            }
        }
    }
}
