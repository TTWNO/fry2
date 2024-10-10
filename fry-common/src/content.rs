//! CST Content struct.

use crate::Feature;

use alloc::vec::Vec;

/// Content struct containing both features and relations (both are just lists of features)
#[derive(Clone, Debug, PartialEq)]
pub struct Content<'a> {
    pub(crate) features: Vec<Feature<'a>>,
    pub(crate) relations: Vec<Feature<'a>>,
}
