//! CST Utterance.

use crate::{
    Feature,
    item::FeatureValue,
};

use alloc::{
    vec::Vec,
    collections::BTreeSet,
};

/// An utterance.
#[derive(Clone, Debug, PartialEq)]
pub struct Utterance<'a> {
    pub(crate) features: Vec<Feature<'a>>,
    pub(crate) ffunctions: Vec<Feature<'a>>,
    pub(crate) relations: BTreeSet<Feature<'a>>,
}
impl Utterance<'_> {
    fn us_f0_model(&mut self) {
        if self.features.feature_present("no_f0_target_model") {
            return;
        }
        todo!()
    }
}
