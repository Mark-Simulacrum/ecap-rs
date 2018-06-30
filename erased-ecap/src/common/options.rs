use ecap;
use ecap::common::{Area, Name, NamedValueVisitor};

pub trait Options {
    fn option(&self, name: &Name) -> Option<&Area>;
    fn visit_each(&self, visitor: &mut dyn NamedValueVisitor);
}

impl<U> Options for U
where
    U: ecap::common::Options + ?Sized,
{
    fn option(&self, name: &Name) -> Option<&Area> {
        U::option(self, name)
    }

    fn visit_each(&self, visitor: &mut dyn NamedValueVisitor) {
        U::visit_each(self, visitor)
    }
}

impl<'a> ecap::common::Options for dyn Options + 'a {
    fn option(&self, name: &Name) -> Option<&Area> {
        self.option(name)
    }

    fn visit_each<V: NamedValueVisitor>(&self, mut visitor: V) {
        self.visit_each(&mut visitor)
    }
}

impl<'a> ecap::common::Options for &'a (dyn Options + 'a) {
    fn option(&self, name: &Name) -> Option<&Area> {
        <Self as Options>::option(self, name)
    }

    fn visit_each<V: NamedValueVisitor>(&self, mut visitor: V) {
        <Self as Options>::visit_each(self, &mut visitor)
    }
}
