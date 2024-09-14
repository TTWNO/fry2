//! Tone CART Tree.

use fry_common::{
    cart_tree::{CartNode, CartOperation, CartTree},
    Value,
};

const CTNODE_NO_0000: usize = 2;
const CTNODE_NO_0002: usize = 4;
const CTNODE_NO_0004: usize = 6;
const CTNODE_NO_0006: usize = 8;
const CTNODE_NO_0008: usize = 10;
const CTNODE_NO_0010: usize = 12;
const CTNODE_NO_0012: usize = 14;
const CTNODE_NO_0014: usize = 16;
const CTNODE_NO_0016: usize = 18;
const CTNODE_NO_0019: usize = 21;
const CTNODE_NO_0021: usize = 23;
const CTNODE_NO_0023: usize = 25;
const CTNODE_NO_0025: usize = 27;
const CTNODE_NO_0027: usize = 29;
const CTNODE_NO_0029: usize = 31;
const CTNODE_NO_0031: usize = 33;
const CTNODE_NO_0033: usize = 35;
const CTNODE_NO_0035: usize = 37;
const CTNODE_NO_0039: usize = 41;
const CTNODE_NO_0038: usize = 42;
const CTNODE_NO_0042: usize = 44;
const CTNODE_NO_0037: usize = 45;
const CTNODE_NO_0018: usize = 46;
const CTNODE_NO_0046: usize = 48;
const CTNODE_NO_0048: usize = 50;
const CTNODE_NO_0051: usize = 53;
const CTNODE_NO_0053: usize = 55;
const CTNODE_NO_0055: usize = 57;
const CTNODE_NO_0057: usize = 59;
const CTNODE_NO_0059: usize = 61;
const CTNODE_NO_0061: usize = 63;
const CTNODE_NO_0063: usize = 65;
const CTNODE_NO_0065: usize = 67;
const CTNODE_NO_0067: usize = 69;
const CTNODE_NO_0069: usize = 71;
const CTNODE_NO_0071: usize = 73;
const CTNODE_NO_0073: usize = 75;
const CTNODE_NO_0075: usize = 77;
const CTNODE_NO_0077: usize = 79;
const CTNODE_NO_0079: usize = 81;
const CTNODE_NO_0081: usize = 83;
const CTNODE_NO_0083: usize = 85;
const CTNODE_NO_0085: usize = 87;
const CTNODE_NO_0087: usize = 89;
const CTNODE_NO_0089: usize = 91;
const CTNODE_NO_0091: usize = 93;
const CTNODE_NO_0050: usize = 94;

const VAL_0000: Value<'static> = Value::Str("1");
const VAL_0001: Value<'static> = Value::Str("H-H%");
const VAL_0002: Value<'static> = Value::Str("cc");
const VAL_0003: Value<'static> = Value::Str("NONE");
const VAL_0004: Value<'static> = Value::Str("10");
const VAL_0005: Value<'static> = Value::Str("md");
const VAL_0006: Value<'static> = Value::Str("4");
const VAL_0007: Value<'static> = Value::Str("det");
const VAL_0008: Value<'static> = Value::Str("3");
const VAL_0009: Value<'static> = Value::Str("in");
const VAL_0010: Value<'static> = Value::Str("0");
const VAL_0011: Value<'static> = Value::Str("L-L%");
const VAL_0012: Value<'static> = Value::Str("aux");
const VAL_0013: Value<'static> = Value::Str("6");
const VAL_0014: Value<'static> = Value::Str("L-H%");
const VAL_0015: Value<'static> = Value::Str("5");
const VAL_0016: Value<'static> = Value::Str("2");
const VAL_0017: Value<'static> = Value::Str("pps");
const VAL_0018: Value<'static> = Value::Str("content");
const VAL_0019: Value<'static> = Value::Str("to");
const VAL_0020: Value<'static> = Value::Str("H-");

const INT_TONE_CART_TREE: CartTree<'static, 95, 15> = CartTree::init_unchecked(
    [
        CartNode::init(0, Some(CartOperation::Is), CTNODE_NO_0000, VAL_0000),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_NO_0002, VAL_0002),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_NO_0004, VAL_0004),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_NO_0006, VAL_0005),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_NO_0008, VAL_0006),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_NO_0010, VAL_0007),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0012, VAL_0008),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0014, VAL_0006),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_NO_0016, VAL_0009),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_NO_0018, VAL_0006),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_NO_0019, VAL_0010),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0021, VAL_0012),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_NO_0023, VAL_0007),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_NO_0025, VAL_0006),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0027, VAL_0013),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_NO_0029, VAL_0012),
        CartNode::init(255, None, 0, VAL_0014),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0031, VAL_0015),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_NO_0033, VAL_0016),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0035, VAL_0007),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_NO_0037, VAL_0010),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_NO_0038, VAL_0009),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0039, VAL_0010),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(255, None, 0, VAL_0014),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0042, VAL_0010),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(255, None, 0, VAL_0014),
        CartNode::init(255, None, 0, VAL_0011),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_NO_0046, VAL_0017),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0048, VAL_0010),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_NO_0050, VAL_0018),
        CartNode::init(11, Some(CartOperation::Is), CTNODE_NO_0051, VAL_0010),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(12, Some(CartOperation::Is), CTNODE_NO_0053, VAL_0006),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0055, VAL_0000),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(13, Some(CartOperation::Is), CTNODE_NO_0057, VAL_0006),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(11, Some(CartOperation::Is), CTNODE_NO_0059, VAL_0009),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0061, VAL_0002),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(13, Some(CartOperation::Is), CTNODE_NO_0063, VAL_0008),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(11, Some(CartOperation::Is), CTNODE_NO_0065, VAL_0002),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_NO_0067, VAL_0010),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(11, Some(CartOperation::Is), CTNODE_NO_0069, VAL_0019),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(14, Some(CartOperation::Is), CTNODE_NO_0071, VAL_0019),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0073, VAL_0009),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(14, Some(CartOperation::Is), CTNODE_NO_0075, VAL_0009),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0077, VAL_0016),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(11, Some(CartOperation::Is), CTNODE_NO_0079, VAL_0012),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_NO_0081, VAL_0000),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(14, Some(CartOperation::Is), CTNODE_NO_0083, VAL_0007),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_NO_0085, VAL_0006),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0087, VAL_0010),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_NO_0089, VAL_0018),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_NO_0091, VAL_0008),
        CartNode::init(255, None, 0, VAL_0020),
        CartNode::init(255, None, 0, VAL_0003),
        CartNode::init(255, None, 0, VAL_0003),
    ],
    [
        "lisp_syl_yn_question",
        "R:SylStructure.parent.gpos",
        "ssyl_in",
        "p.old_syl_break",
        "n.old_syl_break",
        "old_syl_break",
        "R:SylStructure.parent.R:Word.n.gpos",
        "R:SylStructure.parent.R:Word.p.gpos",
        "syl_in",
        "sub_phrases",
        "n.stress",
        "R:SylStructure.parent.R:Word.n.n.gpos",
        "p.p.old_syl_break",
        "n.n.old_syl_break",
        "R:SylStructure.parent.R:Word.p.p.gpos",
    ],
);
