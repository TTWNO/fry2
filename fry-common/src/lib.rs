//! `fry-common`
//!
//! Basic data structures and algorithms for text-to-speech (TTS) processing.
//! Based on:
//! - [Festival](https://github.com/festvox/festival)
//! - [flite](https://github.com/festvox/flite)
//! - [Edinburgh Speech Tools (EST)](https://github.com/festvox/speech_tools)
#![no_std]
#![deny(clippy::all, clippy::pedantic, missing_docs)]
#![allow(clippy::module_name_repetitions)]

extern crate alloc;

pub mod error;
pub mod regex;
pub mod val;
pub mod word;
pub use val::Value;
pub mod cart_tree;
pub use error::Error;
