//! CST Relation.

use crate::{Feature, Utterance};

use alloc::vec::Vec;

/// Relation.
#[derive(Debug, Clone, PartialEq)]
pub struct Relation<'a> {
    pub(crate) name: &'a str,
    pub(crate) features: Vec<Feature<'a>>,
    pub(crate) utterance: Utterance<'a>,
}
