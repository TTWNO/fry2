use alloc::rc::{Rc, Weak};

/// A wrapped T (i.e., it pointed to by either a reference-counted pointer or a weak reference-counted pointer)
///
/// Where the original Flite checks "if (utt == null)" is essentially the same step as checking if the Weak variant of this utterance points to a real utterance (or if it has been deallocated).
/// Although by all intents and purposes this should not happen (in fact it is assumed that it will not happen throughout much of the original codebase), it is at least checked.
#[derive(Debug, Clone)]
pub enum MaybeStrong<T> {
    /// The strong variant contains an owned reference-counted pointer.
    Strong(Rc<T>),
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
    pub fn get(&self) -> Option<Rc<T>> {
        match self {
            MaybeStrong::Strong(s) => Some(Rc::clone(&s)),
            MaybeStrong::Weak(w) => Weak::upgrade(&w),
        }
    }
}

impl<T> PartialEq for MaybeStrong<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Strong(a), Self::Strong(b)) => a == b,
            (Self::Weak(a), Self::Weak(b)) => Weak::upgrade(a) == Weak::upgrade(b),
            (Self::Strong(a), Self::Weak(b)) => Some(a) == Weak::upgrade(b).as_ref(),
            (Self::Weak(a), Self::Strong(b)) => Weak::upgrade(a).as_ref() == Some(b),
        }
    }
}
