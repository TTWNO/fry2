use alloc::rc::{Rc, Weak as RcWeak};
use core::cell::RefCell;

/// A wrapped `Rc<RefCell<T>>` for custom implementations only possible when shared-pointer access is possible.
#[derive(derive_more::Deref, Debug, PartialEq, derive_more::From)]
pub struct Strong<T>(#[deref] Rc<RefCell<T>>);
impl<T> Strong<T> {
    /// Create a new `Strong` reference.
    /// This will create an inner `Rc<RefCell<T>>`
    pub fn new(t: T) -> Strong<T> {
        Strong(Rc::new(RefCell::new(t)))
    }
}
impl<T> Clone for Strong<T> {
    fn clone(&self) -> Strong<T> {
        Strong(Rc::clone(&self.0))
    }
}

/// A wrapped `Weak<RefCell<T>>` for custom implementations only possible when shared-pointer access is possible.
#[derive(Debug, Clone)]
pub struct Weak<T>(RcWeak<RefCell<T>>);

/// A wrapped T (i.e., it pointed to by either a reference-counted pointer or a weak reference-counted pointer)
///
/// Where the original Flite checks "if (utt == null)" is essentially the same step as checking if the Weak variant of this utterance points to a real utterance (or if it has been deallocated).
/// Although by all intents and purposes this should not happen (in fact it is assumed that it will not happen throughout much of the original codebase), it is at least checked.
#[derive(Debug, Clone)]
pub enum MaybeStrong<T> {
    /// The strong variant contains an owned reference-counted pointer.
    Strong(Strong<T>),
    /// The weak variant contains a referenced (unowned) reference-counted pointer of the same type.
    Weak(Weak<T>),
}

impl<T> MaybeStrong<T> {
    /// Get the (optional) reference to the inner `T`.
    /// This method will succeed if it contains a strong reference (`Rc<T>`),
    /// but could fail if there is a `Weak<T>` variant; if this fails, it will return `None`
    ///
    /// Implementation note:
    ///
    /// - In the original Flite code, an explicit reference-count addition is not needed often (due to raw pointers everywhere), but Rust required strict ownership.
    /// - This means that an explicit reference count addition is done upon _every_ invocation of this function on a `MaybeStrong::Strong` varaint.
    pub fn get(&self) -> Option<Strong<T>> {
        match self {
            MaybeStrong::Strong(s) => Some(Strong::clone(&s)),
            MaybeStrong::Weak(w) => Some(Strong(RcWeak::upgrade(&w.0)?)),
        }
    }
}

impl<T> From<Strong<T>> for MaybeStrong<T> {
    fn from(strong: Strong<T>) -> MaybeStrong<T> {
        MaybeStrong::Strong(strong)
    }
}
impl<T> From<Weak<T>> for MaybeStrong<T> {
    fn from(weak: Weak<T>) -> MaybeStrong<T> {
        MaybeStrong::Weak(weak)
    }
}
impl<T> From<RcWeak<RefCell<T>>> for MaybeStrong<T> {
    fn from(weak: RcWeak<RefCell<T>>) -> MaybeStrong<T> {
        MaybeStrong::Weak(Weak(weak))
    }
}

impl<T> PartialEq for MaybeStrong<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Strong(a), Self::Strong(b)) => a == b,
            (Self::Weak(a), Self::Weak(b)) => RcWeak::upgrade(&a.0) == RcWeak::upgrade(&b.0),
            (Self::Strong(a), Self::Weak(b)) => Some(&a.0) == RcWeak::upgrade(&b.0).as_ref(),
            (Self::Weak(a), Self::Strong(b)) => RcWeak::upgrade(&a.0).as_ref() == Some(&b.0),
        }
    }
}
