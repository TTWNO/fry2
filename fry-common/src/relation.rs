//! CST Relation.

use crate::{
    Utterance,
    Feature,
};

use alloc::vec::Vec;

/// Relation.
#[derive(Debug, Clone, PartialEq)]
pub struct Relation<'a> {
    name: &'a str,
    features: Vec<Feature<'a>>,
    utterance: Vec<Utterance<'a>>,
}
