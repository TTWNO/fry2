//! CST Item and a tree containing its nodes.

use crate::{
    Content,
    Relation,
};
use alloc::vec::Vec;

/// An individual item.
#[derive(Clone, Debug)]
pub struct Item<'a> {
    content: Content<'a>,
    relation: Relation<'a>,
}

/// An item node in the tree.
#[derive(Clone, Debug)]
pub struct ItemNode<'a> {
    item: Item<'a>,
    next: usize,
    prev: usize,
    up: usize,
    down: usize,
}

/// An item tree structure.
#[derive(Clone, Debug)]
pub struct ItemTree<'a> {
    items: Vec<ItemNode<'a>>,
}
impl<'a> ItemTree<'a> {
    fn find_path<'b>(feature_path: &'b str, typ: bool) {
        todo!()
    }
}
