use common::{Area, Name};

pub trait NamedValueVisitor {
    fn visit(&mut self, name: &Name, value: &Area);
}

impl<'a, T: ?Sized + NamedValueVisitor> NamedValueVisitor for &'a mut T {
    fn visit(&mut self, name: &Name, value: &Area) {
        (&mut **self).visit(name, value);
    }
}
