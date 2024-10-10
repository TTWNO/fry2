//! CART Trees.
//!
//! See: <https://en.wikipedia.org/wiki/Decision_tree_learning>

use crate::error::CartTreeError;
use crate::Value;

/// A CART node operation.
#[allow(missing_docs)]
#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
pub enum CartOperation {
    Is,
    In,
    Less,
    Greater,
    Matches,
    Equals,
    Leaf,
}

/// NOTE: this could likely be turned into a generic node over T, instead of requiring the `Value`
/// enum.
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub struct CartNode<'a> {
    feature: u8,
    operation: Option<CartOperation>,
    next_node: usize,
    value: Value<'a>,
}
impl<'a> CartNode<'a> {
    /// Initialize a `CartNode`
    #[must_use]
    pub const fn init(
        feature: u8,
        operation: Option<CartOperation>,
        next_node: usize,
        value: Value<'a>,
    ) -> Self {
        Self {
            feature,
            operation,
            next_node,
            value,
        }
    }
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug)]
pub struct CartTree<'a, const RULE_LEN: usize, const FEAT_LEN: usize> {
    rule_table: [CartNode<'a>; RULE_LEN],
    feature_table: [&'a str; FEAT_LEN],
}

impl<'a, const RULE_LEN: usize, const FEAT_LEN: usize> CartTree<'a, RULE_LEN, FEAT_LEN> {
    #[must_use]
    /// Create a `CartTree`.
    ///
    /// # Panics
    ///
    /// If one of the `CartNode`s have an index which in not within bounds.
    pub const fn init_unchecked(
        rule_table: [CartNode<'a>; RULE_LEN],
        feature_table: [&'a str; FEAT_LEN],
    ) -> Self {
        match Self::init(rule_table, feature_table) {
            Ok(s) => s,
            Err(CartTreeError::InvalidIndex(idx)) => {
                const_panic::concat_panic!(
                    "The index ",
                    idx,
                    " is not a valid index for the CART Tree"
                )
            }
        }
    }
    /// Create a `CartTree`.
    ///
    /// # Errors
    ///
    /// If one of the `CartNode`s have an index which in not within bounds.
    pub const fn init(
        rule_table: [CartNode<'a>; RULE_LEN],
        feature_table: [&'a str; FEAT_LEN],
    ) -> Result<Self, CartTreeError> {
        let mut i = 0;
        while i < RULE_LEN {
            if rule_table[i].next_node > RULE_LEN {
                return Err(CartTreeError::InvalidIndex(i));
            }
            i += 1;
        }
        Ok(Self {
            rule_table,
            feature_table,
        })
    }
}
