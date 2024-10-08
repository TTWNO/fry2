//! CST Value based on `inclue/cst_val.h` in _Flite_

use crate::{
    Utterance,
    Relation,
    Item,
    CartTree,
    Feature,
};
use alloc::vec::Vec;

#[repr(u8)]
#[derive(PartialEq, Debug, Clone)]
/// A generic value, which could be a `String`, `Int` (16 bits), or `Float` (32 bits)
pub enum Value<'a> {
    /// A string with a lifetime
    Str(&'a str),
    /// An integer: signed, 16 bits
    Int(i16),
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
    /// TODO: item
    Item(&'a Item<'a>) = 21,
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
