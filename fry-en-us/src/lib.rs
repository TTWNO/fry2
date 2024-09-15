//! Translation of the algorithms from the `lang/usenglish/` directory in the flite text-to-speech
//! engine.
#![no_std]
#![deny(clippy::all, clippy::pedantic, clippy::cargo, rustdoc::all)]
#![allow(clippy::module_name_repetitions, clippy::struct_field_names)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod aswd;
mod aswd_state;
pub mod dur_stats;
mod durz_cart;
pub mod expand;
pub mod fzero_lr_term;
mod int_accent_cart;
mod int_tone_cart;
mod nums_cart;
pub mod phoneset;
mod phrasing_cart;
pub mod pos;
mod pos_cart;
