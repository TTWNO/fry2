//! Word has many additional functionalites availadle beyond that of a standard string.
//! See trait implementation blocks for more information.

use alloc::string::String;
use derive_more::{Deref, From};

/// A word is a custom defined [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html),    
/// whith allows `fry` to implement custom funtionalty on the string,
/// without removing the convenience of standard library usage.
#[derive(Deref, From)]
pub struct Word(String);
