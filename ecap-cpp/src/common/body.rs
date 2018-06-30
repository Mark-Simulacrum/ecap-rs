use ffi;

use ecap::common::Body;

foreign_ref!(pub struct CppBody(ffi::Body));

impl Body for CppBody {
    fn size(&self) -> Option<u64> {
        unimplemented!()
    }
}
