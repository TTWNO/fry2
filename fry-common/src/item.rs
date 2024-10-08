//! CST Item and a tree containing its nodes.

use crate::{
    Content,
    Relation,
    Path,
    Feature,
    Value,
};
use alloc::vec::Vec;
use indextree::{NodeId, Arena};

/// An individual item.
#[derive(Clone, Debug)]
pub struct Item<'a> {
    content: Content<'a>,
    relation: Relation<'a>,
}
impl<'a> Item<'a> {
    fn feature_value<'b>(&'b self, name: &'b str) -> Option<&'b Feature<'b>> {
        if let Some(feat) = self.content.relations.iter().find(|feature| feature.name == name) {
            return Some(feat);
        } else {
            return None;
        }
    }
}

/// A full item tree.
struct ItemTree<'a>(pub Arena<Item<'a>>);

impl<'a> ItemTree<'a> {
    fn find_feature(&self, node: NodeId, multipath: &'a str) {
        let paths = multipath.split(":.");
        
    }
}

