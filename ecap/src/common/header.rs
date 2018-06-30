use common::{Area, Name, NamedValueVisitor};

use common::Version;

// FIXME: Should we be using `http` crate types?
//
// Maybe. It would allow easier interop, but also imposes standard Rust
// library types that don't fit the libecap model as well.

/// This represents a header structure.
///
/// It contains many fields, and is essentially a map of Name to Area.
pub trait Header<'a> {
    /// Returns true if this header has at least one field with the specified Name.
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
    // XXX: don't erase the type here
    fn visit_each(&self, visitor: &mut dyn NamedValueVisitor);

    /// Area view on the underlying buffer representing this header.
    fn image(&self) -> &Area;

    /// Parses a given buffer into this Header.
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

    fn protocol(&self) -> &Name;
    fn set_protocol(&mut self, protocol: Name);
}

/// The URI and method, e.g. "GET /".
///
/// This is only present in requests.
pub trait RequestLine: FirstLine {
    fn uri(&self) -> &Area;
    fn set_uri(&mut self, area: Area);

    fn method(&self) -> &Name;
    fn set_method(&mut self, name: Name);
}

/// The status code and reason phrase, e.g. "200 OK".
///
/// This is only present in responses.
pub trait StatusLine: FirstLine {
    fn status_code(&self) -> u16;
    fn set_status_code(&mut self, code: u16);

    fn reason_phrase(&self) -> &Name;
    fn set_reason_phrase(&mut self, name: Name);
}
