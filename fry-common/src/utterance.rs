//! CST Utterance.

use crate::{maybe_strong::Strong, Features, Relation, Value};

use alloc::{collections::BTreeSet, vec::Vec};

/// An utterance.
#[derive(Debug, PartialEq)]
pub struct Utterance<'a> {
    pub(crate) features: Features<'a>,
    pub(crate) ffunctions: Features<'a>,
    pub(crate) relations: Features<'a>,
}
impl<'a> Strong<Utterance<'a>> {
	fn relation_create(&mut self, name: &'a str, value: Value<'a>) -> &'a Relation<'a> {
		let mstr_utt = self.clone().into();
		let mut utt = self.borrow_mut();
		let rel = Relation::new(name, mstr_utt);
		let rel_val = rel.into();
		utt.relations.set(name, rel_val);
		rel_val.borrow().relation().unwrap()
	}
}
