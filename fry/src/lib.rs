//! `fry`
//!
//! A re-implementation of the [`flite`](https://github.com/festvox/flite) TTS engine.
//! Although, this also borrows some code from [Festival](https://github.com/festvox/festival), since flite occasionally has some ["magic numbers"](https://en.wikipedia.org/wiki/Magic_number_(programming)) to reduce runtime overhead.
//! This crate is `no_std` compatible.

#![no_std]
