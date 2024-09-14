//! Accent CART Tree.

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
const CTNODE_NO_0018: usize = 20;
const CTNODE_NO_0020: usize = 22;
const CTNODE_NO_0022: usize = 24;
const CTNODE_NO_0024: usize = 26;
const CTNODE_NO_0026: usize = 28;
const CTNODE_NO_0029: usize = 31;
const CTNODE_NO_0028: usize = 32;
const CTNODE_NO_0033: usize = 35;
const CTNODE_NO_0035: usize = 37;
const CTNODE_NO_0037: usize = 39;
const CTNODE_NO_0039: usize = 41;
const CTNODE_NO_0041: usize = 43;
const CTNODE_NO_0043: usize = 45;
const CTNODE_NO_0032: usize = 46;
const CTNODE_NO_0046: usize = 48;
const CTNODE_NO_0048: usize = 50;
const CTNODE_NO_0051: usize = 53;
const CTNODE_NO_0050: usize = 54;
const CTNODE_NO_0056: usize = 58;
const CTNODE_NO_0058: usize = 60;
const CTNODE_NO_0061: usize = 63;
const CTNODE_NO_0060: usize = 64;
const CTNODE_NO_0055: usize = 65;
const CTNODE_NO_0065: usize = 67;
const CTNODE_NO_0054: usize = 68;
const CTNODE_NO_0069: usize = 71;
const CTNODE_NO_0068: usize = 72;
const CTNODE_NO_0073: usize = 75;
const CTNODE_NO_0075: usize = 77;
const CTNODE_NO_0078: usize = 80;
const CTNODE_NO_0077: usize = 81;
const CTNODE_NO_0072: usize = 82;
const CTNODE_NO_0084: usize = 86;
const CTNODE_NO_0083: usize = 87;
const CTNODE_NO_0082: usize = 88;
const CTNODE_NO_0089: usize = 91;
const CTNODE_NO_0088: usize = 92;
const CTNODE_NO_0093: usize = 95;
const CTNODE_NO_0095: usize = 97;
const CTNODE_NO_0097: usize = 99;
const CTNODE_NO_0099: usize = 101;
const CTNODE_NO_0092: usize = 102;
const CTNODE_NO_0103: usize = 105;
const CTNODE_NO_0105: usize = 107;
const CTNODE_NO_0102: usize = 108;
const CTNODE_NO_0108: usize = 110;
const CTNODE_NO_0111: usize = 113;
const CTNODE_NO_0110: usize = 114;
const CTNODE_NO_0116: usize = 118;
const CTNODE_NO_0115: usize = 119;
const CTNODE_NO_0114: usize = 120;
const CTNODE_NO_0120: usize = 122;
const CTNODE_NO_0122: usize = 124;
const CTNODE_NO_0124: usize = 126;
const CTNODE_NO_0127: usize = 129;
const CTNODE_NO_0129: usize = 131;
const CTNODE_NO_0131: usize = 133;
const CTNODE_NO_0133: usize = 135;
const CTNODE_NO_0126: usize = 136;
const CTNODE_NO_0136: usize = 138;
const CTNODE_NO_0138: usize = 140;
const CTNODE_NO_0140: usize = 142;
const CTNODE_NO_0142: usize = 144;
const CTNODE_NO_0145: usize = 147;
const CTNODE_NO_0144: usize = 148;

const VAL_0000: Value<'static> = Value::Str("1");
const VAL_0001: Value<'static> = Value::Str("H*");
const VAL_0002: Value<'static> = Value::Str("NONE");
const VAL_0003: Value<'static> = Value::Str("10");
const VAL_0004: Value<'static> = Value::Str("to");
const VAL_0005: Value<'static> = Value::Str("cc");
const VAL_0006: Value<'static> = Value::Str("in");
const VAL_0007: Value<'static> = Value::Str("wp");
const VAL_0008: Value<'static> = Value::Str("aux");
const VAL_0009: Value<'static> = Value::Str("det");
const VAL_0010: Value<'static> = Value::Str("0");
const VAL_0011: Value<'static> = Value::Str("md");
const VAL_0012: Value<'static> = Value::Str("3");
const VAL_0013: Value<'static> = Value::Str("4");
const VAL_0014: Value<'static> = Value::Str("content");
const VAL_0015: Value<'static> = Value::Str("2");
const VAL_0016: Value<'static> = Value::Str("L+H*");
const VAL_0017: Value<'static> = Value::Str("5");
const VAL_0018: Value<'static> = Value::Str("!H*");
const VAL_0019: Value<'static> = Value::Str("7");
const VAL_0020: Value<'static> = Value::Str("6");

static INT_ACCENT_CART_TREE: CartTree<'static, 149, 18> = CartTree::init_unchecked(
    [
        CartNode::init(0, Some(CartOperation::Is), CTNODE_NO_0000, VAL_0000),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_NO_0002, VAL_0000),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_NO_0004, VAL_0000),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_NO_0006, VAL_0003),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0008, VAL_0004),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0010, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_NO_0012, VAL_0003),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0014, VAL_0006),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0016, VAL_0007),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0018, VAL_0008),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0020, VAL_0009),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_NO_0022, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0024, VAL_0010),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_NO_0026, VAL_0011),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0028, VAL_0012),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_NO_0029, VAL_0000),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0032, VAL_0013),
        CartNode::init(11, Some(CartOperation::Is), CTNODE_NO_0033, VAL_0006),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(12, Some(CartOperation::Is), CTNODE_NO_0035, VAL_0010),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(13, Some(CartOperation::Is), CTNODE_NO_0037, VAL_0006),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_NO_0039, VAL_0010),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(14, Some(CartOperation::Is), CTNODE_NO_0041, VAL_0010),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0043, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_NO_0046, VAL_0010),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_NO_0048, VAL_0013),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0050, VAL_0012),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0051, VAL_0014),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(15, Some(CartOperation::Is), CTNODE_NO_0054, VAL_0015),
        CartNode::init(14, Some(CartOperation::Is), CTNODE_NO_0055, VAL_0010),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0056, VAL_0006),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(12, Some(CartOperation::Is), CTNODE_NO_0058, VAL_0015),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0060, VAL_0014),
        CartNode::init(12, Some(CartOperation::Is), CTNODE_NO_0061, VAL_0013),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(16, Some(CartOperation::Is), CTNODE_NO_0065, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(13, Some(CartOperation::Is), CTNODE_NO_0068, VAL_0010),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_NO_0069, VAL_0010),
        CartNode::init(255, None, 0, VAL_0016),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_NO_0072, VAL_0000),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_NO_0073, VAL_0012),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0075, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(14, Some(CartOperation::Is), CTNODE_NO_0077, VAL_0010),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0078, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(12, Some(CartOperation::Is), CTNODE_NO_0082, VAL_0013),
        CartNode::init(17, Some(CartOperation::Is), CTNODE_NO_0083, VAL_0010),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0084, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0016),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(12, Some(CartOperation::Is), CTNODE_NO_0088, VAL_0017),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0089, VAL_0014),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(11, Some(CartOperation::Is), CTNODE_NO_0092, VAL_0006),
        CartNode::init(15, Some(CartOperation::Is), CTNODE_NO_0093, VAL_0013),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(15, Some(CartOperation::Is), CTNODE_NO_0095, VAL_0012),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0097, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0099, VAL_0014),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(255, None, 0, VAL_0018),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_NO_0102, VAL_0015),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0103, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0105, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0016),
        CartNode::init(11, Some(CartOperation::Is), CTNODE_NO_0108, VAL_0004),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_NO_0110, VAL_0012),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0111, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(15, Some(CartOperation::Is), CTNODE_NO_0114, VAL_0019),
        CartNode::init(14, Some(CartOperation::Is), CTNODE_NO_0115, VAL_0010),
        CartNode::init(13, Some(CartOperation::Is), CTNODE_NO_0116, VAL_0014),
        CartNode::init(255, None, 0, VAL_0018),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(13, Some(CartOperation::Is), CTNODE_NO_0120, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(17, Some(CartOperation::Is), CTNODE_NO_0122, VAL_0012),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_NO_0124, VAL_0013),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_NO_0126, VAL_0010),
        CartNode::init(16, Some(CartOperation::Is), CTNODE_NO_0127, VAL_0000),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_NO_0129, VAL_0012),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(10, Some(CartOperation::Is), CTNODE_NO_0131, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_NO_0133, VAL_0000),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_NO_0136, VAL_0006),
        CartNode::init(255, None, 0, VAL_0016),
        CartNode::init(15, Some(CartOperation::Is), CTNODE_NO_0138, VAL_0020),
        CartNode::init(255, None, 0, VAL_0018),
        CartNode::init(12, Some(CartOperation::Is), CTNODE_NO_0140, VAL_0015),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(15, Some(CartOperation::Is), CTNODE_NO_0142, VAL_0013),
        CartNode::init(255, None, 0, VAL_0018),
        CartNode::init(14, Some(CartOperation::Is), CTNODE_NO_0144, VAL_0010),
        CartNode::init(15, Some(CartOperation::Is), CTNODE_NO_0145, VAL_0012),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0002),
        //CartNode::init( 255, None, 0, 0): I don't think this is necessary; it looks like a
        //terminating NULL value in the value field
    ],
    // I have no idea what the hell these are! But they're in the file -_-
    [
        "R:SylStructure.parent.R:Token.parent.EMPH",
        "n.R:SylStructure.parent.R:Token.parent.EMPH",
        "p.R:SylStructure.parent.R:Token.parent.EMPH",
        "ssyl_in",
        "R:SylStructure.parent.gpos",
        "ssyl_out",
        "stress",
        "R:SylStructure.parent.R:Word.p.gpos",
        "p.syl_break",
        "syl_break",
        "p.p.syl_break",
        "R:SylStructure.parent.R:Word.p.p.gpos",
        "syl_out",
        "R:SylStructure.parent.R:Word.n.gpos",
        "n.stress",
        "syl_in",
        "n.syl_break",
        "n.n.syl_break",
    ],
);
