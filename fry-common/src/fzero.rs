//! `F0` is a measure of frequency across time as the lowest common frequency across the period
//!
//! See also: <https://en.wikipedia.org/wiki/Fundamental_frequency>

use crate::item::Item;
use itertools::Itertools;
use core::iter::once;

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
/// Appliy an LR model and convert to a set of floats represeting the (start, mid, end).
/// It returns None if there is not at least one item in the `f0_lr_terms` iterator.
/// http://www.festvox.org/docs/manual-2.4.0/festival_25.html#Linear-regression
pub fn apply_lr_model<'b>(item: &'b Item, mut f0_lr_terms: impl Iterator<Item = &'b FZero<'b>>) -> Option<(f32, f32, f32)> {
    // set interceptors
    let icp = f0_lr_terms.next()?;
    let mut start = icp.start;
    let mut mid = icp.mid;
    let mut end = icp.end;

    // create iterator that has `icp` as the first element
    once(icp)
        .chain(f0_lr_terms)
        // overlapping windows: (0,1),(1,2),(2,3)
        .tuple_windows()
        .map(|(last, cur)| {
        });

    todo!()
}
