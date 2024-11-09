use fry_common::{
    cart_tree::{CartNode, CartOperation, CartTree},
    ValueInner,
};
const CTNODE_US_PHRASING_NO_0000: usize = 2;
const CTNODE_US_PHRASING_NO_0004: usize = 6;
const CTNODE_US_PHRASING_NO_0006: usize = 8;
const CTNODE_US_PHRASING_NO_0003: usize = 9;
const CTNODE_US_PHRASING_NO_0002: usize = 10;
const CTNODE_US_PHRASING_NO_0010: usize = 12;
const CTNODE_US_PHRASING_NO_0012: usize = 14;

const VAL_0000: ValueInner<'static> = ValueInner::Str("--");
const VAL_0001: ValueInner<'static> = ValueInner::Str("BB");
const VAL_0002: ValueInner<'static> = ValueInner::Str("0");
const VAL_0003: ValueInner<'static> = ValueInner::Str("");
const VAL_0004: ValueInner<'static> = ValueInner::Str("1");
const VAL_0005: ValueInner<'static> = ValueInner::Str("NB");

fn phrasing_cart_tree() -> CartTree<'static, 15, 6> {
    CartTree::init_unchecked(
        [
            CartNode::init_val_inner(
                0,
                Some(CartOperation::Is),
                CTNODE_US_PHRASING_NO_0000,
                VAL_0000,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0001),
            CartNode::init_val_inner(
                1,
                Some(CartOperation::Is),
                CTNODE_US_PHRASING_NO_0002,
                VAL_0002,
            ),
            CartNode::init_val_inner(
                2,
                Some(CartOperation::Is),
                CTNODE_US_PHRASING_NO_0003,
                VAL_0003,
            ),
            CartNode::init_val_inner(
                3,
                Some(CartOperation::Is),
                CTNODE_US_PHRASING_NO_0004,
                VAL_0004,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0001),
            CartNode::init_val_inner(
                4,
                Some(CartOperation::Is),
                CTNODE_US_PHRASING_NO_0006,
                VAL_0004,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0001),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0001),
            CartNode::init_val_inner(
                5,
                Some(CartOperation::Is),
                CTNODE_US_PHRASING_NO_0010,
                VAL_0002,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0001),
            CartNode::init_val_inner(
                4,
                Some(CartOperation::Is),
                CTNODE_US_PHRASING_NO_0012,
                VAL_0004,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0001),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
        ],
        [
            "R:Token.parent.n.name",
            "R:Token.n.name",
            "R:Token.parent.punc",
            "R:Token.parent.break",
            "break",
            "n.name",
        ],
    )
}

#[test]
fn test_phrasing_cart_tree() {
    let x = phrasing_cart_tree();
}
