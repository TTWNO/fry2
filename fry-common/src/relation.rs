//! CST Relation.

use crate::{Feature, MaybeStrong, Utterance};

use alloc::vec::Vec;

/// Relation.
#[derive(Debug, PartialEq)]
pub struct Relation<'a> {
    pub(crate) name: &'a str,
    pub(crate) features: Vec<Feature<'a>>,
    pub(crate) utterance: MaybeStrong<Utterance<'a>>,
}
