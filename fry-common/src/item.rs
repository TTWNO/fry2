//! CST Item and a tree containing its nodes.

use crate::{Content, Feature, Path, Relation, Utterance, Value};
use alloc::{
    rc::{Rc, Weak},
    str,
    vec::Vec,
};
use core::ops::{ControlFlow, Deref};
use indextree::{Arena, NodeEdge, NodeId};
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

trait GetNodeId {
    fn node_id(&self) -> NodeId;
}
impl GetNodeId for NodeEdge {
    fn node_id(&self) -> NodeId {
        match self {
            NodeEdge::Start(nid) | NodeEdge::End(nid) => *nid,
        }
    }
}

/// An individual item's content.
#[derive(Clone, Debug, PartialEq)]
pub struct ItemContents<'a> {
    features: Vec<Feature<'a>>,
    relations: Vec<Feature<'a>>,
}

/// An individual item.
#[derive(Clone, Debug, PartialEq)]
pub struct Item<'a> {
    contents: ItemContents<'a>,
    relation: Option<Relation<'a>>,
}

fn feature_value<'a>(
    mut iter: impl Iterator<Item = &'a Feature<'a>>,
    name: &'a str,
) -> Option<&'a Value<'a>> {
    Some(&iter.find(|feat| feat.name == name)?.value)
}

/// A full item tree.
pub struct ItemTree<'a>(pub Arena<Item<'a>>);

impl<'a> ItemTree<'a> {
    /// Grab an item from the tree.
    #[must_use]
    pub fn get(&'a self, node: NodeId) -> Option<&'a Item<'a>> {
        Some(self.0.get(node)?.get())
    }
    fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.0.get(node)?.parent()
    }
    fn next_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.0.get(node)?.next_sibling()
    }
    fn prev_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.0.get(node)?.previous_sibling()
    }
    fn first_child(&self, node: NodeId) -> Option<NodeId> {
        self.0.get(node)?.first_child()
    }
    fn second_child(&self, node: NodeId) -> Option<NodeId> {
        self.0.get(self.0.get(node)?.first_child()?)?.next_sibling()
    }
    fn last_child(&self, node: NodeId) -> Option<NodeId> {
        self.0.get(node)?.last_child()
    }
    fn relation(&self, node: NodeId, name: &'a str) -> Option<NodeId> {
        self.0
            .get(node)
            .map(|node| feature_value(node.get().contents.relations.iter(), name)?.item())?
    }
    fn next(&self, node: NodeId) -> Option<NodeId> {
        Some(node.traverse(&self.0).nth(1)?.node_id())
    }
    fn next_next(&self, node: NodeId) -> Option<NodeId> {
        Some(node.traverse(&self.0).nth(2)?.node_id())
    }
    fn first(&self, node: NodeId) -> Option<NodeId> {
        Some(node.traverse(&self.0).last()?.node_id())
    }
    fn previous(&self, node: NodeId) -> Option<NodeId> {
        Some(node.reverse_traverse(&self.0).nth(1)?.node_id())
    }
    fn previous_previous(&self, node: NodeId) -> Option<NodeId> {
        Some(node.reverse_traverse(&self.0).nth(2)?.node_id())
    }
    fn last(&self, node: NodeId) -> Option<NodeId> {
        Some(node.reverse_traverse(&self.0).last()?.node_id())
    }
    fn use_path(&self, node: NodeId, path: &'a Path<'a>) -> Option<NodeId> {
        // note: traverse starts at the current node, so skip(1) is used to go to the next one
        match path {
            Path::Next => self.next(node),
            Path::Previous => self.previous(node),
            Path::First => self.first(node),
            Path::Last => self.last(node),
            Path::Parent => self.parent(node),
            Path::NextNext => self.next_next(node),
            Path::PreviousPrevious => self.previous_previous(node),
            Path::Daughter => self.first_child(node),
            Path::SecondDaughter => self.second_child(node),
            Path::LastDaughter => self.last_child(node),
            Path::Relation(rel) => self.relation(node, rel),
        }
    }
    /// Get the item at the end of the feature path.
    ///
    /// NOTE: xref `src/hrg/ffeatures.c:internal_ff`
    pub fn find_feature(&self, node: NodeId, multipath: &'a str) -> Option<Value<'a>> {
        let dest: NodeId = multipath
            .split('.')
            .map(Path::try_from)
            // TODO: is there a way to do this without collecing?
            .collect::<Result<Vec<Path>, _>>()
            .ok()?
            .iter()
            // NOTE: must drop last item; I'd prefer something like `skip_last` but haven't found it anywhere, but this is good enough
            .tuple_windows()
            .map(|(a, b)| a)
            // graph traversal, use item from first path as input to next section of path
            // NOTE: if any use_path directive fails (returns None) it will short-circurit the rest
            // of the function
            .try_fold(node, |nid, path| self.use_path(nid, path))?;
        // yes we have to parse it twice. Can't figure aut how to combine this in the big piping
        // arrangement above
        let last_path = multipath.split('.').last()?;
        let dest_item: &Item<'a> = self.0.get(dest)?.get();
        // NOTE: does not have the ffunc condition in the original ffeature function
        // this is because it's a function pointer pass; pretty sure we don't need it
        Some(
            // if the last item in the path is found in the feature set
            feature_value(dest_item.contents.features.iter(), last_path)
                // then the destination is the right value
                .map(|_| Value::Item(dest))
                // otherwise use the default value (Value::Int(0))
                .unwrap_or_default(),
        )
    }
    /// Path to an item via its mulitpath
    // TODO: src/hrg/cst_ffeature.c:154-183
    #[must_use]
    pub fn path_to_item(&self, node: NodeId, mulitpath: &'a str) -> Option<Item<'a>> {
        todo!()
    }
}
