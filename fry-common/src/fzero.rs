//! `F0` is a measure of frequency across time as the lowest common frequency across the period
//!
//! See also: https://en.wikipedia.org/wiki/Fundamental_frequency

/// Struct that holds an `F0` term.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct FZero<'a> {
    feature: &'a str,
    start: f32,
    mid: f32,
    end: f32,
    typ: Option<&'a str>,
}

impl<'a> FZero<'a> {
    /// Initialize the `F0` term.
    pub const fn init(
        feature: &'a str,
        start: f32,
        mid: f32,
        end: f32,
        typ: Option<&'a str>,
    ) -> Self {
        Self {
            feature,
            start,
            mid,
            end,
            typ,
        }
    }
}
