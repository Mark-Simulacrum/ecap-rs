use ecap;
use ecap::common::{Area, Name, NamedValueVisitor};

pub trait Header {}

impl<T: ?Sized> Header for T
where
    T: ecap::common::header::Header,
{
}

impl ecap::common::header::Header for dyn Header {
    fn contains_field(&self, field: &Name) -> bool {
        Self::contains_field(self, field)
    }
    fn get(&self, field: &Name) -> Option<&Area> {
        Self::get(self, field)
    }
    fn insert(&mut self, field: Name, value: Area) {
        Self::insert(self, field, value)
    }
    fn remove_any(&mut self, field: &Name) {
        Self::remove_any(self, field)
    }
    fn visit_each<V: NamedValueVisitor>(&self, visitor: &mut V) {
        Self::visit_each(self, visitor)
    }
    fn image(&self) -> &Area {
        Self::image(self)
    }
    fn parse(&mut self, buf: &Area) -> Result<(), ()> {
        Self::parse(self, buf)
    }
}
