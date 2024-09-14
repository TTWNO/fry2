//! CST Value based on `inclue/cst_val.h` in _Flite_

#[derive(PartialEq, Debug)]
/// A generic value, which could be a `String`, `Int` (16 bits), or `Float` (32 bits)
pub enum Value<'a> {
    /// A string with a lifetime
    Str(&'a str),
    /// An integer: signed, 16 bits
    Int(i16),
    /// A float
    Float(f32),
}
