//! Phone Set
//!
//! I have no idea what this really is? But I'm making it anyways.

use crate::Value;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Phoneset<'a> {
    name: &'a str,
    feature_names: &'a [&'a str],
    feature_values: &'a [Value<'a>],
    phone_names: &'a [&'a str],
    silence: &'a str,
    fv_table: &'a [[usize; 11]],
}

impl<'a> Phoneset<'a> {
    /// Finds the index of the matching name in the `phone_names` slice
    /// If not found, 0 is returned; this is fine since the index can still be used.
    fn phone_id(&self, name: &'a str) -> usize {
        self.phone_names
            .iter()
            .position(|s| *s == name)
            .unwrap_or_default()
    }
    /// Finds the index of the matching name in the `feature_names` slice
    /// If not found, 0 is returned; this is fine since the index can still be used.
    fn phone_feature_id(&self, name: &'a str) -> usize {
        self.feature_names
            .iter()
            .position(|s| *s == name)
            .unwrap_or_default()
    }
    pub fn phone_feature(&self, phone_name: &'a str, feat_name: &'a str) -> Option<&'a Value<'a>> {
        self.feature_values
            .get(
                *(self.fv_table
                    .get(self.phone_id(phone_name))?
                    .get(self.phone_feature_id(feat_name))?)
            )
    }
}
