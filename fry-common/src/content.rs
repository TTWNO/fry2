//! CST Content struct.

use crate::Feature;

use alloc::vec::Vec;

/// Content struct containing both features and relations (both are just lists of features)
#[derive(Clone, Debug)]
pub struct Content<'a> {
    features: Vec<Feature<'a>>,
    relations: Vec<Feature<'a>>,
}
