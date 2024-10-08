//! Path components for representing relations between tree items.

use strum::EnumString;

/// A single path component (where to go to next) in a tree.
///
/// http://www.festvox.org/docs/manual-2.4.0/festival_14.html#Features
#[derive(PartialEq, Eq, EnumString)]
#[allow(missing_docs)]
pub enum Path {
    #[strum(serialize="n")]
    Next,
    #[strum(serialize="p")]
    Previous,
    #[strum(serialize="first")]
    First,
    #[strum(serialize="last")]
    Last,
    #[strum(serialize="parent")]
    Parent,
    #[strum(serialize="nn")]
    NextNext,
    #[strum(serialize="pp")]
    PreviousPrevious,
    #[strum(serialize="daughter",serialize="daughter1")]
    Daughter,
    #[strum(serialize="daughter2")]
    SecondDaughter,
    #[strum(serialize="daughtern")]
    LastDaughter,
    #[strum(serialize="R")]
    Relation,
}


