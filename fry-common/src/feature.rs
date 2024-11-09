//! A pair of values.

use crate::Value;
use alloc::vec::Vec;

/// A borrowed `str` and a `crate::Value` with the same lifetime.
#[derive(Debug, PartialEq)]
pub struct Feature<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: Value<'a>,
}

/// A set of features, which have unique features beyond that of a plain `Vec<Feature>`
#[derive(Debug, PartialEq)]
pub struct Features<'a> {
    inner: Vec<Feature<'a>>,
}
impl<'a> Features<'a> {
    /// Get the value of an individual feature, if a feature with the name `name` is found.
    /// None otherwise.
    pub fn feature_value(&self, name: &str) -> Option<&Value<'a>> {
        Some(&self.inner.iter().find(|feat| feat.name == name)?.value)
    }
    /// Check if a feature exists in this set.
    /// True if the feature name is found, false otherwise.
    pub fn feature_present(&self, name: &str) -> bool {
        self.inner.iter().any(|feat| feat.name == name)
    }
}
