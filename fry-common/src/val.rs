//! CST Value based on `inclue/cst_val.h` in _Flite_

use crate::{error::ValueError, Features, Float, Int, Phoneset, Relation, Strong, Utterance};
use alloc::boxed::Box;
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
impl<'a> core::ops::Deref for Value<'a> {
    type Target = ValueAtom<'a>;
    fn deref(&self) -> &ValueAtom<'a> {
        match self {
            Value::Cons((ref atom, _)) | Value::Atom(ref atom) => atom,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, EnumDiscriminants, derive_more::From)]
#[strum_discriminants(derive(Display))]
/// A generic value, which could be a `String`, `Int` (16 bits), or `Float` (32 bits)
pub enum ValueAtom<'a> {
    /// A string with a lifetime
    Str(&'a str),
    /// An integer: signed, 32 bits
    /// may need to be 64 bits? idx
    Int(Int),
    /// A float
    Float(Float),
    /// Utterance
    Utterance(Strong<Utterance<'a>>) = 7,
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
    Relation(Strong<Relation<'a>>) = 19,
    /// TODO: item; encoded as a `NodeId` so that it can grab the Item from the arena
    Item(NodeId) = 21,
    ///// TODO: cart tree
    //Cart(&'a CartTree<'a, 1, 1>) = 23,
    /// TODO: phoneset
    Phoneset(Strong<Phoneset<'a>>) = 25,
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
    Features(Strong<Features<'a>>) = 45,
    //// TODO: breakfunc
    //BreakFunc(()) = 47,
    //// TODO: `cg_db`
    //ClustergenDb(()) = 49,
    //// TODO: voice
    //Voice(()) = 51,
    //// TODO: `audio_streaming_info`
    //AudioStreamingInfo(()) = 53,
}
impl<'a> ValueAtom<'a> {
    /// Gets the `Relation` value if exists, `None` otherwise
    #[must_use]
    pub fn relation(&'a self) -> Option<&'a Strong<Relation<'a>>> {
        let ValueAtom::Relation(rel) = self else {
            return None;
        };
        Some(rel)
    }
    /// Gets the `Phoneset` value if exists, `None` otherwise
    #[must_use]
    pub fn phoneset(&'a self) -> Option<&'a Strong<Phoneset<'a>>> {
        let ValueAtom::Phoneset(ph) = self else {
            return None;
        };
        Some(ph)
    }
    /// Gets `str` inner value, `None` otherwise
    #[must_use]
    pub fn str(&self) -> Option<&'a str> {
        let ValueAtom::Str(s) = self else {
            return None;
        };
        Some(s)
    }
    /// Gets `item` inner value, `None` otherwise
    #[must_use]
    pub fn item(&self) -> Option<NodeId> {
        let ValueAtom::Item(id) = self else {
            return None;
        };
        Some(*id)
    }
    /// Get the `Float` inner value, `None` otherwise
    /// Works for either an int (will cast to float), or string (will parse float)
    ///
    /// # Errors
    ///
    /// - If the `ValueAtom` is any variant other than:
    ///     - Float
    ///     - Int, or
    ///     - Str
    pub fn float(&self) -> Result<Float, ValueError> {
        match self {
            Self::Float(f) => Ok(*f),
            Self::Int(i) => Ok(*i as Float),
            Self::Str(s) => Ok(Float::from_str(s)?),
            _ => Err(ValueError::InvalidType {
                orig: self.into(),
                try_to: ValueAtomDiscriminants::Float,
            }),
        }
    }
}
impl<'a> Default for ValueAtom<'a> {
    fn default() -> ValueAtom<'a> {
        ValueAtom::Int(0)
    }
}
impl<'a> PartialEq<Value<'a>> for Value<'a> {
    fn eq(&self, other: &Value<'a>) -> bool {
        **self == **other
    }
}
impl<'a, T> PartialEq<T> for Value<'a>
where
    ValueAtom<'a>: PartialEq<T>,
    T: ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        match self {
            Value::Atom(ref atom) | Value::Cons((ref atom, _)) => atom == other,
        }
    }
}
impl PartialEq<str> for ValueAtom<'_> {
    fn eq(&self, other: &str) -> bool {
        let ValueAtom::Str(s) = &self else {
            return false;
        };
        *s == other
    }
}
