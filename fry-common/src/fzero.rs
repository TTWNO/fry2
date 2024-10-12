//! `F0` is a measure of frequency across time as the lowest common frequency across the period
//!
//! See also: <https://en.wikipedia.org/wiki/Fundamental_frequency>

use crate::item::{Item, ItemTree};
use core::iter::once;
use indextree::NodeId;
use itertools::Itertools;

/// Struct that holds an `F0` term.
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub struct FZero<'a> {
    feature: &'a str,
    start: f32,
    mid: f32,
    end: f32,
    typ: Option<&'a str>,
}

impl<'a> FZero<'a> {
    /// Initialize the `F0` term.
    #[must_use]
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
///
/// It returns None if there is not at least one item in the `f0_lr_terms` iterator.
/// <http://www.festvox.org/docs/manual-2.4.0/festival_25.html#Linear-regression>
pub fn apply_lr_model<'b>(
    tree: &'b ItemTree<'b>,
    node: NodeId,
    mut f0_lr_terms: impl Iterator<Item = &'b FZero<'b>>,
) -> Option<(f32, f32, f32)> {
    // set interceptors
    let icp = f0_lr_terms.next()?;
    let mut start = icp.start;
    let mut mid = icp.mid;
    let mut end = icp.end;
    let mut i_val = None;

    // create iterator that has `icp` as the first element
    Some(
        once(icp)
        .chain(f0_lr_terms)
        // overlapping windows: (0,1),(1,2),(2,3)
        .tuple_windows()
        .map(|(last, cur)| {
            if last.feature != cur.feature {
                i_val = tree.find_feature(node, cur.feature);
            }

            // TODO: make this a mapping pipeline
            let fv = if let Some(val) = &i_val {
                if let Some(typ) = cur.typ {
                    if *val == *typ {
                        1.0
                    } else {
                        0.0
                    }
                } else {
                    // NOTE: if this operation fails in Flite, the program crashes by calling
                    // cst_error, which is a macro.
                    // The macro has this comment for if it's for PalmOS not using the ARM aritechture: "I've never tested this or even compiled it"
                    // we decidede to pass back up a Result
                    // NOTE: this condition is not in `flite`; it makes sense to have a 0 condition just in
                    // case thought
                    val.float().unwrap_or_default()
                }
            // NOTE: this condition is not in `flite`; it makes sense to have a 0 condition just in
            // case thought
            } else {
                0.0
            };
            (fv, cur.start, cur.mid, cur.end)
        })
        .map(|(fv, c_start, c_mid, c_end)| {
            (fv*c_start, fv*c_mid, fv*c_end)
        })
        .fold((start, mid, end), |(s, m, e), (c_s, c_m, c_e)| {
            (s + c_s, m + c_m, e + c_e)
        })
    )
}
