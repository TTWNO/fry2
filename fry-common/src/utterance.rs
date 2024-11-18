//! CST Utterance.

use crate::{maybe_strong::Strong, Features, MaybeStrong, Relation, Value, ValueAtom};
use tap::{Pipe, Tap};

use alloc::rc::Rc;

/// An utterance.
#[derive(Debug, PartialEq)]
pub struct Utterance<'a> {
    pub(crate) features: Features<'a>,
    pub(crate) ffunctions: Features<'a>,
    pub(crate) relations: Features<'a>,
}
impl<'a> Utterance<'a> {
    fn relation(&'a self, name: &'a str) -> Option<&'a Value<'a>> {
        self.relations.feature_value(name)
    }
}
impl<'a> Strong<Utterance<'a>> {
    fn relation_create(&mut self, name: &'a str) -> Strong<Relation<'a>> {
        let (rel_val, rel_copy) = Rc::downgrade(self)
            .pipe(MaybeStrong::from)
            .pipe(|mstr_utt| Relation::new(name, mstr_utt))
            .pipe(Strong::new)
            .pipe(|st_rel| {
                (
                    Strong::clone(&st_rel)
                        .pipe(ValueAtom::from)
                        .pipe(Value::from),
                    st_rel,
                )
            });
        self.borrow_mut()
            .tap_mut(|utt| utt.relations.set(name, rel_val));
        rel_copy
    }
}
