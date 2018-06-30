use common::{Area, Name, NamedValueVisitor};

/// Reading of `(Name, Area)` pairs across the adapter/host boundary.
///
/// This is used to share configuration information and transaction meta-information.
///
/// FIXME: "Options objects and individual option values may be temporary. They must not
/// be used beyond the method call that supplied or asked for them." -- what does this mean?
pub trait Options {
    /// Returns the value of the named option.
    ///
    /// `None` is returned if unknown or nonexistant.
    fn option(&self, name: &Name) -> Option<Area>;

    /// Calls visitor for each `(Name, Area)` pair.
    ///
    /// Accesses all options, including those whose `Name` is unknown.
    fn visit_each(&self, visitor: &mut dyn NamedValueVisitor);
}
