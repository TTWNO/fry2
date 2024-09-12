//! Translation of the algorithms from the `lang/usenglish/` directory in the flite text-to-speech
//! engine.
#![no_std]
#![deny(clippy::all, clippy::pedantic, clippy::cargo, rustdoc::all)]
#![allow(clippy::module_name_repetitions, clippy::struct_field_names)]
#![forbid(unsafe_code)]

pub mod aswd;
mod aswd_state;
pub mod dur_stats;
pub mod expand;
