//! CST Utterance.

use crate::Feature;

use alloc::vec::Vec;

/// An utterance.
#[derive(Clone, Debug, PartialEq)]
pub struct Utterance<'a> {
    features: Vec<Feature<'a>>,
    ffunctions: Vec<Feature<'a>>,
    relations: Vec<Feature<'a>>,
}
