//! A pair of values.

use crate::Value;

/// A borrowed `str` and a `crate::Value` with the same lifetime.
#[derive(Clone, Debug)]
pub struct Feature<'a> {
    name: &'a str,
    value: Value<'a>,
}
