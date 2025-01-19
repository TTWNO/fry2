//! `fry-common`
//!
//! Basic data structures and algorithms for text-to-speech (TTS) processing.
//! Based on:
//! - [Festival](https://github.com/festvox/festival)
//! - [flite](https://github.com/festvox/flite)
//! - [Edinburgh Speech Tools (EST)](https://github.com/festvox/speech_tools)
#![no_std]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    rustdoc::all,
    //missing_docs
)]
#![allow(clippy::module_name_repetitions, dead_code)]

extern crate alloc;

mod maybe_strong;
pub use maybe_strong::{MaybeStrong, Strong, Weak};

pub mod int;
pub use int::Int;
pub mod float;
pub use float::Float;
pub mod item;
pub use item::Item;
pub mod relation;
use relation::Relation;
pub mod utterance;
use utterance::Utterance;
pub mod feature;
use feature::Features;
pub mod content;
mod lex_data;
use lex_data::LEX_DATA;
pub mod lexicon;
use lexicon::Lexicon;
pub mod lts_rules;
use lts_rules::LtsRules;
pub mod path;
use path::Path;
mod phone_huff_table;
use phone_huff_table::LEX_PHONE_HUFF_TABLE;
pub mod phoneme;
use phoneme::{Phoneme, Stress};
pub mod phoneset;
use phoneset::Phoneset;
pub mod diphone;
pub use diphone::DiphoneEntry;
pub mod error;
pub mod regex;
pub mod val;
pub mod word;
pub use val::{Value, ValueAtom};
pub mod cart_tree;
pub use cart_tree::CartTree;
pub use error::Error;
pub mod fzero;
pub use fzero::FZero;
