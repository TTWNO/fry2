# `fry`

A re-implementation of the `flite` TTS engine in pure-Rust.

This is **really early** so there is no API or docs to speak of.
These will be filled in as the project nears completion.

## Optimizations That Could Be Done

- All `CartTree`s could be re-written as Rust code instead of a data stucture. Using either a) a scheme interpreter macro b) a `build.rs` file, or c) changing the `make_cart` Scheme file in `flite` to output Rust code.
- SIMD for speech synth functions (still TODO).

