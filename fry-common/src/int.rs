//! Int width used for all calculations.
//!
//! Defaults to [`i64`], but can be forced to [`i32`] if on a 32-bit platform via the `32-bit` flag.

/// The universal int type for the codebase.
/// Necessary because Rust does not allow `i64` to be added to by an `i32`, so we need to define a specific width for the entire codebase.
#[cfg(not(feature = "32-bit"))]
pub type Int = i64;

/// The universal int type for the codebase.
/// Necessary because Rust does not allow `i64` to be added to by an `i32`, so we need to define a specific width for the entire codebase.
#[cfg(feature = "32-bit")]
pub type Int = i32;
