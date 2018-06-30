use common::{Area, Name};

pub trait NamedValueVisitor {
    fn visit(&mut self, name: &Name, value: &Area);
}

impl<F: FnMut(&Name, &Area)> NamedValueVisitor for F {
    fn visit(&mut self, name: &Name, value: &Area) {
        (self)(name, value)
    }
}
