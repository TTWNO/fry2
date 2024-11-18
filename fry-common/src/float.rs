//! Float width used for all calculations.
//!
//! Defaults to [`f64`], but can be forced to [`f32`] if on a 32-bit platform via the `f32` flag.

/// The universal float type for the codebase.
/// Necessary because Rust does not allow `f64` to be added to by an `f32`, so we need to define a specific width for the entire codebase.
#[cfg(not(feature = "f32"))]
pub type Float = f64;

/// The universal float type for the codebase.
/// Necessary because Rust does not allow `f64` to be added to by an `f32`, so we need to define a specific width for the entire codebase.
#[cfg(feature = "f32")]
pub type Float = f32;
