use std::sync::atomic::{Ordering, AtomicUsize};
use std::cell::Cell;
use std::borrow::Cow;

static LAST_ID: AtomicUsize = AtomicUsize::new(0);

/// Representation of a protocol token constant or similar name.
///
/// This is optimized for a set of globally known unique IDs and
/// associated strings through associating an integer key with these
/// IDs.
///
/// FIXME: Rust does not have great support for global, mutable,
/// constant data that is initialized consecutively. It is unclear as
/// such how best to implement these IDs (other than trivially doing so
/// via having a list and manually 0, 1, 2, 3... then setting the
/// LAST_ID global to the last id from that list.
///
/// A given name can also be associated by the host with some `u32` ID,
/// which will persist across adapter boundary.
#[derive(Clone)]
pub struct Name {
    image: Option<Cow<'static, str>>,
    id: Id,
    host_id: Cell<Option<u32>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Id {
    Unknown, // 0
    Unidentified, // 1
    Id(u32),
}


impl Name {
    pub fn unknown() -> Name {
        Name {
            image: None,
            id: Id::Unknown,
            host_id: Cell::new(None),
        }
    }

    pub fn new_known(image: String) -> Name {
        Name {
            image: Some(image.into()),
            id: Id::Unidentified,
            host_id: Cell::new(None),
        }
    }

    pub fn new_identified<I: Into<Cow<'static, str>>>(image: I) -> Name {
        Name {
            image: Some(image.into()),
            id: Id::Id(LAST_ID.fetch_add(1, Ordering::Relaxed) as u32),
            host_id: Cell::new(None),
        }
    }

    /// Is this name associated with an identifier?
    ///
    /// All global constant names will be associated with an identifier.
    ///
    /// FIXME: Squid does not use this API; neither do adapters. It's
    /// not clear what the use case is.
    pub fn identified(&self) -> bool {
        if let Id::Id(_) = self.id {
            true
        } else {
            false
        }
    }

    /// Is this `Name` known?
    ///
    /// A name is known if it was created with a string.
    ///
    /// FIXME: This API seems non-useful as `image` returns `Option`.
    ///     Squid only uses this API to check if the image is empty...
    ///     though why it can't do that directly isn't known (i.e.,
    ///     use image().size() == 0).
    ///
    /// FIXME: This is bad naming, too
    pub fn known(&self) -> bool {
        if let Id::Unknown = self.id {
            false
        } else {
            true
        }
    }

    /// Retrieves the string representation of this `Name`.
    ///
    /// It may not exist, in which case this will return `None`.
    pub fn image(&self) -> Option<&str> {
        self.image.as_ref().map(|s| s.as_ref())
    }

    /// Retrieves the ID set by the host.
    ///
    /// This should only be called by the host.
    ///
    /// This will panic if called twice on the same `Name`.
    pub fn host_id(&self) -> Option<u32> {
        self.host_id.get()
    }

    /// Assigns a host ID.
    ///
    /// This must only be called by the host.
    ///
    /// This will panic if called twice on the same `Name`.
    pub fn assign_host_id(&self, id: u32) {
        assert_eq!(self.host_id.replace(Some(id)), None);
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.known() &&
        if self.identified() {
            self.id == other.id
        } else {
            self.image == other.image
        }
    }
}

//lazy_static! {
//    pub static ref PROTOCOL_HTTP: Name = Name::new_identified("HTTP");
//    pub static ref PROTOCOL_HTTPS: Name = Name::new_identified("HTTPS");
//    pub static ref PROTOCOL_FTP: Name = Name::new_identified("FTP");
//    pub static ref PROTOCOL_GOPHER: Name = Name::new_identified("GOPHER");
//    pub static ref PROTOCOL_WAIS: Name = Name::new_identified("WAIS");
//    pub static ref PROTOCOL_URN: Name = Name::new_identified("URN");
//    pub static ref PROTOCOL_WHOIS: Name = Name::new_identified("WHOIS");
//
//    pub static ref METHOD_GET: Name = Name::new_identified("GET");
//    pub static ref METHOD_PUT: Name = Name::new_identified("PUT");
//    pub static ref METHOD_POST: Name = Name::new_identified("POST");
//    pub static ref METHOD_HEAD: Name = Name::new_identified("HEAD");
//    pub static ref METHOD_CONNECT: Name = Name::new_identified("CONNECT");
//    pub static ref METHOD_OPTIONS: Name = Name::new_identified("OPTIONS");
//    pub static ref METHOD_DELETE: Name = Name::new_identified("DELETE");
//    pub static ref METHOD_TRACE: Name = Name::new_identified("TRACE");
//
//    pub static ref HEADER_CONTENT_LENGTH: Name = Name::new_identified("Content-Length");
//    pub static ref HEADER_TRANSFER_ENCODING: Name = Name::new_identified("Transfer-Encoding");
//    pub static ref HEADER_REFERER: Name = Name::new_identified("Referer");
//    pub static ref HEADER_VIA: Name = Name::new_identified("Via");
//    pub static ref HEADER_X_CLIENT_IP: Name = Name::new_identified("X-Client-IP");
//    pub static ref HEADER_X_SERVER_IP: Name = Name::new_identified("X-Server-IP");
//
//    pub static ref META_CLIENT_IP: Name = HEADER_X_CLIENT_IP.clone();
//    pub static ref META_SERVER_IP: Name = HEADER_X_SERVER_IP.clone();
//    pub static ref META_USER_NAME: Name = Name::new_identified("X-Client-Username");
//    pub static ref META_AUTHENTICATED_USER: Name = Name::new_identified("X-Authenticated-User");
//    pub static ref META_AUTHENTICATED_GROUPS: Name = Name::new_identified("X-Authenticated-Groups");
//    pub static ref META_SUBSCRIBER_ID: Name = Name::new_identified("X-Subscriber-ID");
//    pub static ref META_VIRUS_ID: Name = Name::new_identified("X-Virus-ID");
//    pub static ref META_RESPONSE_INFO: Name = Name::new_identified("X-Response-Info");
//    pub static ref META_RESPONSE_DESC: Name = Name::new_identified("X-Response-Desc");
//    pub static ref META_NEXT_SERVICES: Name = Name::new_identified("X-Next-Services");
//}