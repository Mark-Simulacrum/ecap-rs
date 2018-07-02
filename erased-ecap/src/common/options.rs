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

impl<'a> ecap::common::Options for &'a (dyn Options + 'a) {
    fn option(&self, name: &Name) -> Option<&Area> {
        <(dyn Options) as Options>::option(&**self, name)
    }

    fn visit_each<V: NamedValueVisitor>(&self, mut visitor: V) {
        <(dyn Options) as Options>::visit_each(&**self, &mut visitor)
    }
}
