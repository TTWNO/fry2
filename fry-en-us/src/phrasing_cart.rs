use fry_common::{
    cart_tree::{CartNode, CartOperation, CartTree},
    Value,
};
const CTNODE_US_PHRASING_NO_0000: usize = 2;
const CTNODE_US_PHRASING_NO_0004: usize = 6;
const CTNODE_US_PHRASING_NO_0006: usize = 8;
const CTNODE_US_PHRASING_NO_0003: usize = 9;
const CTNODE_US_PHRASING_NO_0002: usize = 10;
const CTNODE_US_PHRASING_NO_0010: usize = 12;
const CTNODE_US_PHRASING_NO_0012: usize = 14;

const VAL_0000: Value<'static> = Value::Str("--");
const VAL_0001: Value<'static> = Value::Str("BB");
const VAL_0002: Value<'static> = Value::Str("0");
const VAL_0003: Value<'static> = Value::Str("");
const VAL_0004: Value<'static> = Value::Str("1");
const VAL_0005: Value<'static> = Value::Str("NB");

const PHRASING_CART_TREE: CartTree<'static, 15, 6> = CartTree::init_unchecked(
    [
        CartNode::init(
            0,
            Some(CartOperation::Is),
            CTNODE_US_PHRASING_NO_0000,
            VAL_0000,
        ),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(
            1,
            Some(CartOperation::Is),
            CTNODE_US_PHRASING_NO_0002,
            VAL_0002,
        ),
        CartNode::init(
            2,
            Some(CartOperation::Is),
            CTNODE_US_PHRASING_NO_0003,
            VAL_0003,
        ),
        CartNode::init(
            3,
            Some(CartOperation::Is),
            CTNODE_US_PHRASING_NO_0004,
            VAL_0004,
        ),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(
            4,
            Some(CartOperation::Is),
            CTNODE_US_PHRASING_NO_0006,
            VAL_0004,
        ),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(
            5,
            Some(CartOperation::Is),
            CTNODE_US_PHRASING_NO_0010,
            VAL_0002,
        ),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(
            4,
            Some(CartOperation::Is),
            CTNODE_US_PHRASING_NO_0012,
            VAL_0004,
        ),
        CartNode::init(255, None, 0, VAL_0001),
        CartNode::init(255, None, 0, VAL_0005),
    ],
    [
        "R:Token.parent.n.name",
        "R:Token.n.name",
        "R:Token.parent.punc",
        "R:Token.parent.break",
        "break",
        "n.name",
    ],
);
