//! CST Utterance.

use crate::{item::FeatureValue, Feature, maybe_strong::Strong};

use alloc::{collections::BTreeSet, vec::Vec};

/// An utterance.
#[derive(Clone, Debug, PartialEq)]
pub struct Utterance<'a> {
    pub(crate) features: Vec<Feature<'a>>,
    pub(crate) ffunctions: Vec<Feature<'a>>,
    pub(crate) relations: BTreeSet<Feature<'a>>,
}
impl Strong<Utterance<'_>> {
    fn us_f0_model(&mut self) {
        if self.features.feature_present("no_f0_target_model") {
            return;
        }
        todo!()
    }
}
