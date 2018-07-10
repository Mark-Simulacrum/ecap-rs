use std::borrow::Cow;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

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
///
/// XXX: Debug impl
#[derive(Debug, Clone)]
pub struct Name<'a> {
    image: Option<Cow<'a, [u8]>>,
    id: Id,
    host_id: Cell<Option<u32>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Id {
    Unknown,      // 0
    Unidentified, // 1
    Id(u32),
}

impl<'a> Name<'a> {
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn to_owned(self) -> Name<'static> {
        Name {
            id: self.id,
            host_id: self.host_id,
            image: match self.image {
                Some(cow) => Some(Cow::from(cow.into_owned())),
                None => None,
            },
        }
    }

    pub fn from_raw<I: Into<Cow<'a, [u8]>>>(image: I, id: Id, host_id: Option<u32>) -> Self {
        let image = image.into();
        Name {
            image: if image.is_empty() { None } else { Some(image) },
            id,
            host_id: Cell::new(host_id),
        }
    }

    pub fn unknown() -> Name<'static> {
        Name {
            image: None,
            id: Id::Unknown,
            host_id: Cell::new(None),
        }
    }

    pub fn new_known<I: Into<Cow<'a, [u8]>>>(image: I) -> Name<'a> {
        Name {
            image: Some(image.into()),
            id: Id::Unidentified,
            host_id: Cell::new(None),
        }
    }

    pub fn new_identified<I: Into<Cow<'a, [u8]>>>(image: I) -> Name<'a> {
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
    pub fn image(&self) -> Option<&[u8]> {
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

impl<'a> PartialEq for Name<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.known() && if self.identified() {
            self.id == other.id
        } else {
            self.image == other.image
        }
    }
}
