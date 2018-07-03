use ecap;
use ecap::common::Version;
use ecap::common::{Area, Name, NamedValueVisitor};

pub trait Header {
    fn contains_field(&self, field: &Name) -> bool;
    fn get(&self, field: &Name) -> Option<&Area>;
    fn remove_any(&mut self, field: &Name);
    fn insert(&mut self, field: Name, value: Area);
    fn visit_each(&self, visitor: &mut dyn NamedValueVisitor);
    fn image(&self) -> &Area;
    fn parse(&mut self, buf: &Area) -> Result<(), ()>;
}

impl<T: ?Sized> Header for T
where
    T: ecap::common::header::Header,
{
    fn contains_field(&self, field: &Name) -> bool {
        self.contains_field(field)
    }
    fn get(&self, field: &Name) -> Option<&Area> {
        self.get(field)
    }
    fn remove_any(&mut self, field: &Name) {
        self.remove_any(field)
    }
    fn insert(&mut self, field: Name, value: Area) {
        self.insert(field, value)
    }
    fn visit_each(&self, mut visitor: &mut dyn NamedValueVisitor) {
        self.visit_each(&mut visitor)
    }
    fn image(&self) -> &Area {
        self.image()
    }
    fn parse(&mut self, buf: &Area) -> Result<(), ()> {
        self.parse(buf)
    }
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

pub trait FirstLine {}

impl<T: ?Sized> FirstLine for T
where
    T: ecap::common::header::FirstLine,
{
}

impl ecap::common::header::FirstLine for dyn FirstLine {
    fn version(&self) -> Version {
        unimplemented!()
    }
    fn set_version(&mut self, version: Version) {
        unimplemented!()
    }

    fn protocol(&self) -> &Name {
        unimplemented!()
    }
    fn set_protocol(&mut self, protocol: Name) {
        unimplemented!()
    }
}
