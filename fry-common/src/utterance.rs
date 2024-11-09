//! CST Utterance.

use crate::{item::FeatureValue, maybe_strong::Strong, Feature};

use alloc::{collections::BTreeSet, vec::Vec};

/// An utterance.
#[derive(Debug, PartialEq)]
pub struct Utterance<'a> {
    pub(crate) features: Vec<Feature<'a>>,
    pub(crate) ffunctions: Vec<Feature<'a>>,
    pub(crate) relations: BTreeSet<Feature<'a>>,
}
impl Strong<Utterance<'_>> {
    fn us_f0_model(&mut self) {
        if self.borrow().features.feature_present("no_f0_target_model") {
            return;
        }
        todo!()
    }
}
