//! A pair of values.

use crate::Value;
use alloc::vec::Vec;

/// A borrowed `str` and a `crate::Value` with the same lifetime.
#[derive(Debug, PartialEq)]
pub struct Feature<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: Value<'a>,
}
impl<'a> Feature<'a> {
    /// Create a new feature pair!
    pub fn new(name: &'a str, value: Value<'a>) -> Self {
        Feature { name, value }
    }
}

/// A set of features, which have unique features beyond that of a plain `Vec<Feature>`
#[derive(Debug, PartialEq)]
pub struct Features<'a> {
    inner: Vec<Feature<'a>>,
}
impl<'a> Features<'a> {
    /// Create a new, empty list of features.
    #[must_use]
    pub fn new() -> Self {
        Features { inner: Vec::new() }
    }
    /// Set a feature.
    /// If a feature has the same name, the value will be replaced.
    pub fn set(&mut self, name: &'a str, val: Value<'a>) {
        let Some(idx) = self.inner.iter().position(|feat| feat.name == name) else {
            self.inner.push(Feature::new(name, val));
            return;
        };
        let Some(mut feat) = self.inner.get_mut(idx) else {
            // Technically should never happen, but ok.
            return;
        };
        feat.value = val;
    }
    /// Get the value of an individual feature, if a feature with the name `name` is found.
    /// None otherwise.
    #[must_use]
    pub fn feature_value(&self, name: &str) -> Option<&Value<'a>> {
        Some(&self.inner.iter().find(|feat| feat.name == name)?.value)
    }
    /// Check if a feature exists in this set.
    /// True if the feature name is found, false otherwise.
    #[must_use]
    pub fn feature_present(&self, name: &str) -> bool {
        self.inner.iter().any(|feat| feat.name == name)
    }
}
impl Default for Features<'_> {
    fn default() -> Self {
        Self::new()
    }
}
