//! `fry-common`
//!
//! Basic data structures and algorithms for text-to-speech (TTS) processing.
//! Based on:
//! - [Festival](https://github.com/festvox/festival)
//! - [flite](https://github.com/festvox/flite)
//! - [Edinburgh Speech Tools (EST)](https://github.com/festvox/speech_tools)
#![no_std]
#![forbid(clippy::all, clippy::pedantic, missing_docs)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

pub mod regex;
pub mod word;
