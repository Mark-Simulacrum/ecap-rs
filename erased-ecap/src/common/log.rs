use ecap;
use mopa::Any;
use std::fmt;

pub trait DebugStream: fmt::Write + Any {
    // nothing
}

mopafy!(DebugStream);

impl<U> DebugStream for U
where
    U: ecap::common::log::DebugStream + 'static,
{
}

impl ecap::common::log::DebugStream for dyn DebugStream {}
impl ecap::common::log::DebugStream for Box<dyn DebugStream> {}

impl fmt::Write for Box<dyn DebugStream> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        (&mut **self).write_str(s)
    }
}
