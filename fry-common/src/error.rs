//! All error types across Fry.
//! These are used in all downstream crates, including a lot of internal errors that basically are
//! only used by developers of this crate.
//!
//! These errors are more-or-less original creations; compared to the rest of the crate, which is
//! based on `flite` and `Festival`. Since neither uses their own error types.

use crate::val::ValueDiscriminants;
use derive_more::{Display, Error as DeriveError, From};

/// An error for the `fry-en-us` FSM (finite state machine).
#[derive(Clone, Copy, Debug, Display, DeriveError, From)]
pub enum FsmError {
    /// The FSM references an index which does not exist.
    FsmInvalidIndex(FsmInvalidIndex),
    /// An error related to an individual state of the FSM.
    FsmStateError(FsmStateError),
}

/// An error related to individual states within an FSM.
#[derive(Clone, Copy, Debug, Display, DeriveError)]
pub enum FsmStateError {
    /// The char is not ASCII.
    #[display("NonAscii: the state must contain a valid ASCII char; invalid char: {_0}")]
    #[error(ignore)]
    NonAscii(char),
}

/// An error from the creation of a CART Tree.
#[derive(Clone, Copy, Debug, Display, DeriveError)]
pub enum CartTreeError {
    /// An invalid index was used in a node.
    #[display("InvalidIndex: the node contains an index {_0} to a non-existant node")]
    #[error(ignore)]
    InvalidIndex(usize),
}

/// An error from the conversion from/to a `Value` type.
#[derive(Clone, Debug, Display, DeriveError, From)]
pub enum ValueError {
    /// An invalid type was used for conversion
    #[display("Invalid type: {orig} tried to convert to {try_to}")]
    InvalidType {
        /// the variant of the `Value` type
        orig: ValueDiscriminants,
        /// the variant that was attempted to convert to
        try_to: ValueDiscriminants,
    },
    /// A float conversion error during the conversion between string to float
    #[display("{_0}")]
    FloatParse(core::num::ParseFloatError),
}

/// A Fry error.
#[derive(Clone, Debug, Display, DeriveError, From)]
pub enum Error {
    /// Errors related to the `fry-en-us` FSMs.
    Fsm(FsmError),
    /// Errors related to CART Tree initialization.
    CartTree(CartTreeError),
    /// Errors related to the `Value` enum.
    Value(ValueError),
}

/// When a non-existant index is referenced by an FSM.
#[derive(Clone, Copy, Debug, Display, DeriveError)]
#[display("The index: {ref_idx:} refered to at idx: {idx:} was longer than the length: {len:}")]
pub struct FsmInvalidIndex {
    /// The index which contains the invalid reference.
    pub idx: usize,
    /// The invalid indexed referenced.
    pub ref_idx: usize,
    /// The current length of the FSM.
    pub len: usize,
}
