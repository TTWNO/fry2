//! A pair of values.

use crate::Value;

/// A borrowed `str` and a `crate::Value` with the same lifetime.
#[derive(Debug, PartialEq)]
pub struct Feature<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: Value<'a>,
}
