use common::{Area, Name, NamedValueVisitor, Version};

/// This represents a header structure.
///
/// It contains many fields, and is essentially a map of Name to Area.
pub trait Header {
    /// Returns true if this header has at least one field with the specified Name.
    ///
    /// XXX: This should possibly do something like std's contains methods with Borrow<Self::Name>
    /// instead.
    fn contains_field(&self, field: &Name) -> bool;

    /// Get the specified field(s) by `Name`.
    ///
    /// If multiple headers with the specified field are present, will
    /// return a list of entries separated by ', '.
    ///
    /// This returns an owned `Area` because it may need to allocate in
    /// the list case.
    fn get(&self, field: &Name) -> Option<Area>;

    /// Insert a field, value pair into the header.
    fn insert(&mut self, field: Name, value: Area);

    /// Remove all entries matching the field name.
    fn remove_any(&mut self, field: &Name);

    /// Visit each entry in the header
    fn visit_each<V: NamedValueVisitor>(&self, visitor: &mut V);

    /// Area view on the underlying buffer representing this header.
    fn image(&self) -> Area;

    /// Parses a given buffer into this Header.
    ///
    /// XXX: Should this be Self::Area?
    // XXX: error handling
    fn parse(&mut self, buf: &Area) -> Result<(), ()>;
}

/// The first line in a request/response, e.g. `GET / HTTP/1.1` or
/// `HTTP/1.1 200 OK`.
///
/// See also the [`RequestLine`] and [`StatusLine`] traits.
///
/// XXX: This does not correlate directly in a HTTP 2 scenario?
pub trait FirstLine {
    fn version(&self) -> Version;
    fn set_version(&mut self, version: Version);

    fn protocol(&self) -> Name;
    fn set_protocol(&mut self, protocol: Name);
}

/// The URI and method, e.g. "GET /".
///
/// This is only present in requests.
pub trait RequestLine: FirstLine {
    fn uri(&self) -> Area;
    fn set_uri(&mut self, area: Area);

    fn method(&self) -> Name;
    fn set_method(&mut self, name: Name);
}

/// The status code and reason phrase, e.g. "200 OK".
///
/// This is only present in responses.
pub trait StatusLine: FirstLine {
    fn status_code(&self) -> u16;
    fn set_status_code(&mut self, code: u16);

    fn reason_phrase(&self) -> Name;
    fn set_reason_phrase(&mut self, name: Name);
}
