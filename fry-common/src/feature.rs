//! A pair of values.

use crate::Value;
use indextree::NodeId;

/// A borrowed `str` and a `crate::Value` with the same lifetime.
#[derive(Clone, Debug, PartialEq)]
pub struct Feature<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: Value<'a>,
}
