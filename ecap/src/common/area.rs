use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};
use std::rc::Rc;

/// This is a continous and fixed-size buffer, that can be copied
/// without copying the underlying buffer.
///
/// It does not implement Send/Sync because it may contain a Details
/// object that is not thread-safe, for example [`Rc`](`std::rc::Rc`).
///
/// It can be created from a variety of types, including `Rc<[u8]>`,
/// `Arc<[u8]>`.
///
/// Most of the time you do not want to work with Area itself.
/// Instead, prefer to create it at the boundary between Host/Adapter
/// implementations.
pub struct Area {
    ptr: DetailsStack,
    // Do not implement Send/Sync.
    _data: PhantomData<*mut ()>,
}

impl Area {
    pub fn new<T: DetailsConstructor>(value: T) -> Area {
        Area {
            ptr: value.details(),
            _data: PhantomData,
        }
    }

    /// Create an Area by copying a byte slice.
    pub fn from_bytes(v: &[u8]) -> Area {
        let r: Rc<[u8]> = <Rc<[u8]> as ::std::convert::From<&[u8]>>::from(v);
        Area::new(r)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.ptr.as_bytes()
    }
}

impl Clone for Area {
    fn clone(&self) -> Area {
        self.ptr.increment();
        Area {
            ptr: self.ptr,
            _data: PhantomData,
        }
    }
}

impl Drop for Area {
    fn drop(&mut self) {
        self.ptr.decrement();
    }
}

/// Conversion to a Details object.
///
/// This represents the conversion from some T to a type implementing
/// the `Details` trait.
pub trait DetailsConstructor {
    /// Create the object that will be later used for increment/decrement.
    fn details(self) -> DetailsStack;
}

/// Area content and reference-counting support.
pub trait Details {
    /// Get the underlying buffer.
    fn as_bytes(&self) -> &[u8];

    /// Increment the strong count.
    fn increment(&self);

    /// Decrement the strong count.
    ///
    /// If this is the last Details to be decremented,
    /// this should also deallocate the memory.
    fn decrement(&self);
}

#[derive(Debug)]
struct RcPtr<T: ?Sized> {
    rc: NonNull<T>,
}

impl<T: ?Sized> Clone for RcPtr<T> {
    fn clone(&self) -> RcPtr<T> {
        RcPtr { rc: self.rc }
    }
}

impl<T: ?Sized> Copy for RcPtr<T> {}

impl<T: ?Sized + AsRef<[u8]>> Details for RcPtr<T> {
    fn increment(&self) {
        unsafe {
            let v = Rc::from_raw(self.rc.as_ptr() as *const T);
            mem::forget(v.clone());
            mem::forget(v);
        }
    }

    fn decrement(&self) {
        unsafe {
            mem::drop(Rc::from_raw(self.rc.as_ptr() as *const T));
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { <T as AsRef<[u8]>>::as_ref(self.rc.as_ref()) }
    }
}

impl<T: AsRef<[u8]> + ?Sized + 'static> DetailsConstructor for Rc<T> {
    fn details(self) -> DetailsStack {
        let ptr = Rc::into_raw(self);
        DetailsStack::from(RcPtr {
            rc: NonNull::new(ptr as *mut T).unwrap(),
        })
    }
}

#[derive(Copy, Clone)]
pub struct DetailsStack {
    value: [usize; 2],
    increment: fn(*const ()),
    decrement: fn(*const ()),
    as_bytes: fn(*const ()) -> &'static [u8],
}

impl DetailsStack {
    /// Creates a stack-allocated details trait object and stores the
    /// passed T into it.
    ///
    /// The passed T must fit into `[usize; 2]` and have the same
    /// alignment. This restriction may be relaxed in the future.
    pub fn from<T: Details + Copy + 'static>(v: T) -> DetailsStack {
        let mut data = [0; 2];
        assert!(mem::size_of::<T>() <= mem::size_of::<[usize; 2]>());
        assert_eq!(mem::align_of::<T>(), mem::align_of::<[usize; 2]>());
        assert!(!mem::needs_drop::<T>());
        unsafe {
            ptr::copy_nonoverlapping(&v as *const T, &mut data as *mut [usize; 2] as *mut T, 1);
        }
        mem::forget(v);
        DetailsStack {
            value: data,
            increment: |ptr| unsafe { (&*(ptr as *const T)).increment() },
            decrement: |ptr| unsafe { (&*(ptr as *const T)).decrement() },
            as_bytes: |ptr| unsafe { (&*(ptr as *const T)).as_bytes() },
        }
    }
}

impl Details for DetailsStack {
    fn as_bytes<'a>(&'a self) -> &'a [u8] {
        (self.as_bytes)(&self.value as *const _ as *const ())
    }

    fn increment(&self) {
        (self.increment)(&self.value as *const _ as *const ())
    }

    fn decrement(&self) {
        (self.decrement)(&self.value as *const _ as *const ())
    }
}
