//! CST Item and a tree containing its nodes.

use crate::{Content, Feature, Path, Relation, Utterance, Value, Phoneset};
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

/// Maigc value in `us_f0_model.c`
const MODEL_MEAN: f32 = 170.0;
/// Maigc value in `us_f0_model.c`
const MODEL_STANDARD_DEVIATION: f32 = 34.0;

trait TreeAccess<'a> {
    fn find_feature(&self, tree: &'a ItemTree<'a>, multipath: &'a str) -> Option<Value<'a>>;
    fn in_order_traverse(&self, tree: &'a ItemTree<'a>) -> impl Iterator<Item = (&'a Item<'a>, NodeId)>;
    fn in_order_reverse_traverse(&self, tree: &'a ItemTree<'a>) -> impl Iterator<Item = (&'a Item<'a>, NodeId)>;
    fn parent(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn next_sibling(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn prev_sibling(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn first_child(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn second_child(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn last_child(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn relation(&self, tree: &ItemTree<'a>, name: &'a str) -> Option<NodeId>;
    fn next(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn next_next(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn first(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn previous(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn previous_previous(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
    fn last(&self, tree: &ItemTree<'a>) -> Option<NodeId>;
}
impl<'a> TreeAccess<'a> for NodeId {
    fn find_feature(&self, tree: &'a ItemTree<'a>, multipath: &'a str) -> Option<Value<'a>> {
        tree.find_feature(*self, multipath)
    }
    fn in_order_traverse(&self, tree: &'a ItemTree<'a>) -> impl Iterator<Item = (&'a Item<'a>, NodeId)> {
        tree.traverse(*self)
    }
    fn in_order_reverse_traverse(&self, tree: &'a ItemTree<'a>) -> impl Iterator<Item = (&'a Item<'a>, NodeId)> {
        tree.reverse_traverse(*self)
    }
    fn parent(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.parent(*self)
    }
    fn next_sibling(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.next_sibling(*self)
    }
    fn prev_sibling(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.prev_sibling(*self)
    }
    fn first_child(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.first_child(*self)
    }
    fn second_child(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.second_child(*self)
    }
    fn last_child(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.last_child(*self)
    }
    fn relation(&self, tree: &ItemTree<'a>, name: &'a str) -> Option<NodeId> {
        tree.relation(*self, name)
    }
    fn next(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.next(*self)
    }
    fn next_next(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.next_next(*self)
    }
    fn first(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.first(*self)
    }
    fn previous(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.previous(*self)
    }
    fn previous_previous(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.previous_previous(*self)
    }
    fn last(&self, tree: &ItemTree<'a>) -> Option<NodeId> {
        tree.last(*self)
    }
}

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
impl<'a> Item<'a> {
    fn features(&self) -> &Vec<Feature<'a>> {
        &self.contents.features
    }
    fn relations(&self) -> &Vec<Feature<'a>> {
        &self.contents.relations
    }
    fn utterance(&'a self) -> Option<&'a Utterance<'a>> {
        if let Some(rel) = &self.relation {
            return Some(&rel.utterance);
        }
        None
    }
}

pub(crate) trait FeatureValue<'a> {
    fn feature_value(&self, name: &str) -> Option<&Value<'a>>;
    fn feature_present(&self, name: &str) -> bool;
}
impl<'a> FeatureValue<'a> for [Feature<'a>] {
    fn feature_value(&self, name: &str) -> Option<&Value<'a>> {
        Some(&self.iter().find(|feat| feat.name == name)?.value)
    }
    fn feature_present(&self, name: &str) -> bool {
        self.iter().find(|feat| feat.name == name).is_some()
    }
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
            .map(|node| node.get().relations().feature_value(name)?.item())?
    }
    fn traverse(&'a self, node: NodeId) -> impl Iterator<Item = (&'a Item<'a>, NodeId)> {
        node.traverse(&self.0)
            .filter_map(|ne| Some((self.get(ne.node_id())?, ne.node_id())))
    }
    fn reverse_traverse(&'a self, node: NodeId) -> impl Iterator<Item = (&'a Item<'a>, NodeId)> {
        node.reverse_traverse(&self.0)
            .filter_map(|ne| Some((self.get(ne.node_id())?, ne.node_id())))
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
            dest_item.features()
                .feature_value(last_path)
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
    /// TODO: could be optimized by:
    ///
    /// - converting ""R:SylStructure.daughter.R:Segment.p.name" into an array of `Path` segments
    /// - turn path contests into direct calls .parent().relation(...) etc.
    /// - also applies to many other pieces of code in here
    /// - what does "pau" mean?
    fn pre_break(&self, node: NodeId) -> bool {
        self.next(node).is_none() ||
        if let Some(ref feat) = self.find_feature(node, "R:SylStructure.daughter.R:Segment.p.name") {
            feat == "pau"
        } else { false }
    }
    fn post_break(&self, node: NodeId) -> bool {
        self.previous(node).is_none() ||
        if let Some(ref feat) = self.find_feature(node, "R:SylStructure.daughter.R:Segment.p.name") {
            feat == "pau"
        } else { false }
    }
    fn phoneset(&'a self, node: NodeId) -> Option<&'a Phoneset<'a>> {
        self.get(node)?
            .utterance()?
            .features
            .feature_value("phoneset")?
            .phoneset()
    }
    fn vowel_mid(&'a self, node: NodeId) -> Option<f32> {
        let phone = self.phoneset(node)?;
        self.get(node)?
            .relations()
            .feature_value("SylStructure")?
            .item()?
            .first_child(self)?
            .in_order_traverse(self)
            .filter(|(item, _)| {
                let Some(Value::Str(phone_name)) = item.features().feature_value("name") else {
                    return false;
                };
                let Some(Value::Str(phone_feat_str)) = phone.phone_feature(phone_name, "vc") else {
                    return false;
                };
                "+" == *phone_feat_str
            })
            .filter_map(|(item, id)| Some(
                (item.features()
                    .feature_value("end")?
                    .float()
                    .ok()?
                +
                id.find_feature(self, "R:Segment.p.end")?.float().ok()?)
                /
                2.0
            ))
            .next()
    }
}

const fn map_f0(v: f32, m: f32, s: f32) -> f32 {
    (((v-MODEL_MEAN)/MODEL_STANDARD_DEVIATION)*s)*m
}
