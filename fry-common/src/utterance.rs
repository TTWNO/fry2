//! CST Utterance.

use crate::{maybe_strong::Strong, Features, MaybeStrong, Relation, Value};
use tap::{Pipe, Tap};

use alloc::{collections::BTreeSet, rc::Rc, vec::Vec};
use core::cell::RefCell;

/// An utterance.
#[derive(Debug, PartialEq)]
pub struct Utterance<'a> {
    pub(crate) features: Features<'a>,
    pub(crate) ffunctions: Features<'a>,
    pub(crate) relations: Features<'a>,
}
impl<'a> Strong<Utterance<'a>> {
    fn relation_create(&mut self, name: &'a str, value: Value<'a>) -> Strong<Relation<'a>> {
        let rel = self
            .clone()
            .pipe(MaybeStrong::from)
            .pipe(|mstr_utt| Relation::new(name, mstr_utt))
            .pipe(RefCell::new)
            .pipe(Rc::new)
            .pipe(Strong::from);
        // only an RC clone
        let rel_copy = Strong::clone(&rel);
        // let mut utt = self.borrow_mut();
        //let rel_val = rel.into();
        //utt.relations.set(name, rel_val);
        //rel_val.borrow().relation().unwrap()
        rel_copy
    }
}
