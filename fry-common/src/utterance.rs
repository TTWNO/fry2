//! CST Utterance.

use crate::Feature;

use alloc::vec::Vec;

/// An utterance.
#[derive(Clone, Debug, PartialEq)]
pub struct Utterance<'a> {
    pub(crate) features: Vec<Feature<'a>>,
    pub(crate) ffunctions: Vec<Feature<'a>>,
    pub(crate) relations: Vec<Feature<'a>>,
}
