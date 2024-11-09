//! CST Value based on `inclue/cst_val.h` in _Flite_

use crate::{error::ValueError, Features, Phoneset, Relation, Strong, Utterance};
use alloc::{boxed::Box, vec::Vec};
use core::ops::Deref;
use core::str::FromStr;
use indextree::NodeId;
use strum::{Display, EnumDiscriminants};

/// CST (Carnegie Speech Tools) Value Either:
///
/// - A [`ValueAtom`] in a list, or
/// - A single [`ValueAtom`] value
///
/// [Named `Cons` and `Atom` respectively because of Lisp naming schemes](https://en.wikipedia.org/wiki/Lisp_(programming_language)#Lists)
///
/// This type implements [`core::ops::Deref`] for the inner [`ValueAtom`] structure of either the `Cons` or `Atom` variants.
#[derive(derive_more::From, Debug)]
pub enum Value<'a> {
    /// List variant, first item is data, second is the next item
    Cons((ValueAtom<'a>, Box<Value<'a>>)),
    /// Single value
    Atom(ValueAtom<'a>),
}
impl<'a> From<ValueInner<'a>> for Value<'a> {
    fn from(vi: ValueInner<'a>) -> Value<'a> {
        Value::Atom(ValueAtom::new(vi))
    }
}
impl<'a> core::ops::Deref for Value<'a> {
    type Target = ValueAtom<'a>;
    fn deref(&self) -> &ValueAtom<'a> {
        match self {
            Value::Cons((ref atom, _)) => atom,
            Value::Atom(ref atom) => atom,
        }
    }
}

/// Value atom (AKA a `Strong<Value>`)
#[derive(derive_more::From, derive_more::Deref, PartialEq, Debug)]
pub struct ValueAtom<'a>(#[deref] Strong<ValueInner<'a>>);
impl<'a> Clone for ValueAtom<'a> {
    fn clone(&self) -> ValueAtom<'a> {
        ValueAtom(Strong::clone(&self.0))
    }
}
impl<'a> ValueAtom<'a> {
    fn new(vi: ValueInner<'a>) -> Self {
        ValueAtom(Strong::new(vi))
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, EnumDiscriminants, derive_more::From)]
#[strum_discriminants(derive(Display))]
/// A generic value, which could be a `String`, `Int` (16 bits), or `Float` (32 bits)
pub enum ValueInner<'a> {
    /// A string with a lifetime
    Str(&'a str),
    /// An integer: signed, 32 bits
    /// may need to be 64 bits? idx
    Int(i32),
    /// A float
    Float(f32),
    /// Utterance
    Utterance(Utterance<'a>) = 7,
    ///// TODO: wave
    //Wave(()) = 9,
    ///// TODO: track
    //Track(((),)) = 11,
    ///// TODO: `LPCres`
    //Lpcres(((),(),)) = 13,
    ///// TODO: `UttFunc`
    //UttFunc(((),(),(),)) = 15,
    ///// TODO: ffunc
    //FFunc(((),(),(),(),())) = 17,
    /// TODO: relation
    Relation(Relation<'a>) = 19,
    /// TODO: item; encoded as a `NodeId` so that it can grab the Item from the arena
    Item(NodeId) = 21,
    ///// TODO: cart tree
    //Cart(&'a CartTree<'a, 1, 1>) = 23,
    /// TODO: phoneset
    Phoneset(Phoneset<'a>) = 25,
    //// TODO: lexicon
    //Lexicon(()) = 27,
    //// TODO: durstats
    //DurStats(()) = 29,
    //// TODO: diphonedb
    //DiphoneDb(()) = 31,
    //// TODO: clunitdb
    //ClunitDb(()) = 33,
    //// TODO: `vit_cand`
    //VitCand(()) = 35,
    //// TODO: `sts_list`
    //StsList(()) = 37,
    //// TODO: userdata
    //UserData(()) = 41,
    //// TODO: itemfunc
    //ItemFunc(()) = 43,
    /// TODO: features
    Features(Features<'a>) = 45,
    //// TODO: breakfunc
    //BreakFunc(()) = 47,
    //// TODO: `cg_db`
    //ClustergenDb(()) = 49,
    //// TODO: voice
    //Voice(()) = 51,
    //// TODO: `audio_streaming_info`
    //AudioStreamingInfo(()) = 53,
}
impl<'a> ValueInner<'a> {
    /// Gets the `Phoneset` value if exists, `None` otherwise
    #[must_use]
    pub fn phoneset(&'a self) -> Option<&'a Phoneset<'a>> {
        let ValueInner::Phoneset(ph) = self else {
            return None;
        };
        Some(ph)
    }
    /// Gets `str` inner value, `None` otherwise
    #[must_use]
    pub fn str(&self) -> Option<&'a str> {
        let ValueInner::Str(s) = self else {
            return None;
        };
        Some(s)
    }
    /// Gets `item` inner value, `None` otherwise
    #[must_use]
    pub fn item(&self) -> Option<NodeId> {
        let ValueInner::Item(id) = self else {
            return None;
        };
        Some(*id)
    }
    /// Get the `Float` inner value, `None` otherwise
    /// Works for either an int (will cast to float), or string (will parse float)
    ///
    /// # Errors
    ///
    /// - If the ValueInner is any variant other than:
    ///     - Float
    ///     - Int, or
    ///     - Str
    #[expect(clippy::cast_precision_loss)]
    pub fn float(&self) -> Result<f32, ValueError> {
        match self {
            Self::Float(f) => Ok(*f),
            Self::Int(i) => Ok(*i as f32),
            Self::Str(s) => Ok(f32::from_str(s)?),
            _ => Err(ValueError::InvalidType {
                orig: self.into(),
                try_to: ValueInnerDiscriminants::Float,
            }),
        }
    }
}
impl<'a> Default for ValueInner<'a> {
    fn default() -> ValueInner<'a> {
        ValueInner::Int(0)
    }
}
impl<'a> PartialEq<Value<'a>> for Value<'a> {
    fn eq(&self, other: &Value<'a>) -> bool {
        self.deref() == other.deref()
    }
}
impl<'a, T> PartialEq<T> for Value<'a>
where
    ValueInner<'a>: PartialEq<T>,
    T: ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        let Ok(inner) = self.deref().try_borrow() else {
            return false;
        };
        *inner == *other
    }
}
impl PartialEq<str> for ValueInner<'_> {
    fn eq(&self, other: &str) -> bool {
        let ValueInner::Str(s) = &self else {
            return false;
        };
        *s == other
    }
}
