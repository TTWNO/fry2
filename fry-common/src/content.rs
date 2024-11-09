//! CST Content struct.

use crate::Features;

use alloc::vec::Vec;

/// Content struct containing both features and relations (both are just lists of features)
#[derive(Debug, PartialEq)]
pub struct Content<'a> {
    pub(crate) features: Features<'a>,
    pub(crate) relations: Features<'a>,
}
