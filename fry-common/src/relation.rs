//! CST Relation.

use crate::{Features, MaybeStrong, Utterance};

use alloc::vec::Vec;

/// Relation.
#[derive(Debug, PartialEq)]
pub struct Relation<'a> {
    pub(crate) name: &'a str,
    pub(crate) features: Features<'a>,
    pub(crate) utterance: MaybeStrong<Utterance<'a>>,
}
impl<'a> Relation<'a> {
    /// Create a new, relation set (with an empty features list)
    #[must_use]
    pub fn new(name: &'a str, utterance: MaybeStrong<Utterance<'a>>) -> Self {
        Relation {
            name,
            features: Features::new(),
            utterance,
        }
    }
}
