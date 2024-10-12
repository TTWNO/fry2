//! CST Value based on `inclue/cst_val.h` in _Flite_

use crate::{
    Utterance,
    Relation,
    Item,
    CartTree,
    Feature,
    error::ValueError,
};
use strum::{
    EnumDiscriminants,
    Display,
};
use alloc::vec::Vec;
use core::str::FromStr;
use indextree::NodeId;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(derive(Display))]
/// A generic value, which could be a `String`, `Int` (16 bits), or `Float` (32 bits)
pub enum Value<'a> {
    /// A string with a lifetime
    Str(&'a str),
    /// An integer: signed, 32 bits
    /// may need to be 64 bits? idx
    Int(i32),
    /// A float
    Float(f32),
    /// Utterance
    Utterance(&'a Utterance<'a>) = 7,
    /// TODO: wave
    Wave(()) = 9,
    /// TODO: track
    Track(()) = 11,
    /// TODO: `LPCres`
    Lpcres(()) = 13,
    /// TODO: `UttFunc`
    UttFunc(()) = 15,
    /// TODO: ffunc
    FFunc(()) = 17,
    /// TODO: relation
    Relation(&'a Relation<'a>) = 19,
    /// TODO: item; encoded as a NodeId so that it can grab the Item from the arena
    Item(NodeId) = 21,
    /// TODO: cart tree
    //Cart(&'a CartTree<'a, 1, 1>) = 23,
    /// TODO: phoneset
    Phoneset(()) = 25,
    /// TODO: lexicon
    Lexicon(()) = 27,
    /// TODO: durstats
    //DurStats(()) = 29,
    /// TODO: diphonedb
    DiphoneDb(()) = 31,
    /// TODO: clunitdb
    ClunitDb(()) = 33,
    /// TODO: `vit_cand`
    VitCand(()) = 35,
    /// TODO: `sts_list`
    StsList(()) = 37,
    /// TODO: userdata
    UserData(()) = 41,
    /// TODO: itemfunc
    ItemFunc(()) = 43,
    /// TODO: features
    Features(&'a Vec<Feature<'a>>) = 45,
    /// TODO: breakfunc
    BreakFunc(()) = 47,
    /// TODO: `cg_db`
    ClustergenDb(()) = 49,
    /// TODO: voice
    Voice(()) = 51,
    /// TODO: `audio_streaming_info`
    AudioStreamingInfo(()) = 53,
}
impl<'a> Value<'a> {
    /// Gets `str` inner value, `None` otherwise
    pub fn str(&self) -> Option<&'a str> {
        if let Value::Str(s) = self {
            return Some(s);
        }
        None
    }
    /// Gets `item` inner value, `None` otherwise
    pub fn item(&self) -> Option<NodeId> {
        if let Value::Item(id) = self {
            return Some(*id);
        }
        None
    }
    /// Get the `Float` inner value, `None` otherwise
    /// Works for either an int (will cast to float), or string (will parse float)
    pub fn float(&self) -> Result<f32, ValueError> {
        match self {
            Self::Float(f) => Ok(*f),
            Self::Int(i) => Ok(*i as f32),
            Self::Str(s) => Ok(f32::from_str(s)?),
            _ => Err(ValueError::InvalidType { orig: self.into(), try_to: ValueDiscriminants::Float }),
        }
    }
}
impl<'a> Default for Value<'a> {
    fn default() -> Value<'a> {
        Value::Int(0)
    }
}
