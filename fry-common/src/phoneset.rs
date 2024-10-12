//! Phone Set
//!
//! I have no idea what this really is? But I'm making it anyways.

use crate::Value;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Phoneset<'a> {
    name: &'a str,
    feature_names: &'a [&'a str],
    feature_values: &'a [&'a Value<'a>],
    phone_names: &'a [&'a str],
    silence: &'a str,
    fv_table: &'a [i32],
}
