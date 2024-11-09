//! CST Utterance.

use crate::{maybe_strong::Strong, Features};

use alloc::{collections::BTreeSet, vec::Vec};

/// An utterance.
#[derive(Debug, PartialEq)]
pub struct Utterance<'a> {
    pub(crate) features: Features<'a>,
    pub(crate) ffunctions: Features<'a>,
    pub(crate) relations: Features<'a>,
}
impl Strong<Utterance<'_>> {
    fn us_f0_model(&mut self) {
        if self.borrow().features.feature_present("no_f0_target_model") {
            return;
        }
        todo!()
    }
}
