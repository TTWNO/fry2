use fry_common::{
    cart_tree::{CartNode, CartOperation, CartTree},
    ValueInner,
};

const VAL_0000: ValueInner<'static> = ValueInner::Float(3.8);
const VAL_0001: ValueInner<'static> = ValueInner::Str("month");
const VAL_0002: ValueInner<'static> = ValueInner::Str("0");
const VAL_0003: ValueInner<'static> = ValueInner::Str("year");
const VAL_0004: ValueInner<'static> = ValueInner::Str("ordinal");
const VAL_0005: ValueInner<'static> = ValueInner::Str("cardinal");
const VAL_0006: ValueInner<'static> = ValueInner::Str("numeric");
const VAL_0007: ValueInner<'static> = ValueInner::Float(2.000000);
const VAL_0008: ValueInner<'static> = ValueInner::Str("sym");
const VAL_0009: ValueInner<'static> = ValueInner::Str("digits");
const VAL_0010: ValueInner<'static> = ValueInner::Float(302.3);
const VAL_0011: ValueInner<'static> = ValueInner::Str("flight");
const VAL_0012: ValueInner<'static> = ValueInner::Str("a");
const VAL_0013: ValueInner<'static> = ValueInner::Float(669.2);
const VAL_0014: ValueInner<'static> = ValueInner::Float(373.2);
const VAL_0015: ValueInner<'static> = ValueInner::Float(436.2);
const VAL_0016: ValueInner<'static> = ValueInner::Float(392.6);
const VAL_0017: ValueInner<'static> = ValueInner::Float(716.5);
const VAL_0018: ValueInner<'static> = ValueInner::Float(773.6);
const VAL_0019: ValueInner<'static> = ValueInner::Str("_other_");
const VAL_0020: ValueInner<'static> = ValueInner::Float(4.4);
const VAL_0021: ValueInner<'static> = ValueInner::Float(2_959.6);
const VAL_0022: ValueInner<'static> = ValueInner::Float(1_773.4);
const VAL_0023: ValueInner<'static> = ValueInner::Str("to");
const VAL_0024: ValueInner<'static> = ValueInner::Float(4.6);
const VAL_0025: ValueInner<'static> = ValueInner::Float(4.8);
const VAL_0026: ValueInner<'static> = ValueInner::Float(2880.000000);
const VAL_0027: ValueInner<'static> = ValueInner::Float(1_633.2);
const VAL_0028: ValueInner<'static> = ValueInner::Float(1_306.4);

const CTNODE_US_NUMS_NO_0002: usize = 4;
const CTNODE_US_NUMS_NO_0001: usize = 5;
const CTNODE_US_NUMS_NO_0006: usize = 8;
const CTNODE_US_NUMS_NO_0005: usize = 9;
const CTNODE_US_NUMS_NO_0012: usize = 14;
const CTNODE_US_NUMS_NO_0011: usize = 15;
const CTNODE_US_NUMS_NO_0010: usize = 16;
const CTNODE_US_NUMS_NO_0016: usize = 18;
const CTNODE_US_NUMS_NO_0009: usize = 19;
const CTNODE_US_NUMS_NO_0022: usize = 24;
const CTNODE_US_NUMS_NO_0021: usize = 25;
const CTNODE_US_NUMS_NO_0020: usize = 26;
const CTNODE_US_NUMS_NO_0019: usize = 27;
const CTNODE_US_NUMS_NO_0028: usize = 30;
const CTNODE_US_NUMS_NO_0031: usize = 33;
const CTNODE_US_NUMS_NO_0030: usize = 34;
const CTNODE_US_NUMS_NO_0027: usize = 35;
const CTNODE_US_NUMS_NO_0035: usize = 37;
const CTNODE_US_NUMS_NO_0039: usize = 41;
const CTNODE_US_NUMS_NO_0038: usize = 42;
const CTNODE_US_NUMS_NO_0037: usize = 43;
const CTNODE_US_NUMS_NO_0043: usize = 45;
const CTNODE_US_NUMS_NO_0046: usize = 48;
const CTNODE_US_NUMS_NO_0045: usize = 49;
const CTNODE_US_NUMS_NO_0049: usize = 51;
const CTNODE_US_NUMS_NO_0052: usize = 54;
const CTNODE_US_NUMS_NO_0051: usize = 55;
const CTNODE_US_NUMS_NO_0000: usize = 56;
const CTNODE_US_NUMS_NO_0057: usize = 59;
const CTNODE_US_NUMS_NO_0059: usize = 61;
const CTNODE_US_NUMS_NO_0056: usize = 62;
const CTNODE_US_NUMS_NO_0063: usize = 65;
const CTNODE_US_NUMS_NO_0065: usize = 67;
const CTNODE_US_NUMS_NO_0067: usize = 69;
const CTNODE_US_NUMS_NO_0062: usize = 70;
const CTNODE_US_NUMS_NO_0073: usize = 75;
const CTNODE_US_NUMS_NO_0072: usize = 76;
const CTNODE_US_NUMS_NO_0071: usize = 77;
const CTNODE_US_NUMS_NO_0077: usize = 79;
const CTNODE_US_NUMS_NO_0070: usize = 80;
const CTNODE_US_NUMS_NO_0080: usize = 82;
const CTNODE_US_NUMS_NO_0083: usize = 85;
const CTNODE_US_NUMS_NO_0085: usize = 87;
const CTNODE_US_NUMS_NO_0082: usize = 88;
const CTNODE_US_NUMS_NO_0091: usize = 93;
const CTNODE_US_NUMS_NO_0090: usize = 94;
const CTNODE_US_NUMS_NO_0089: usize = 95;
const CTNODE_US_NUMS_NO_0088: usize = 96;

fn nums_cart_tree() -> CartTree<'static, 97, 7> {
    CartTree::init_unchecked(
        [
            CartNode::init_val_inner(
                0,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0000,
                VAL_0000,
            ),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0001, VAL_0001),
            CartNode::init_val_inner(2, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0002, VAL_0002),
            CartNode::init_val_inner(255, None, 0, VAL_0003),
            CartNode::init_val_inner(255, None, 0, VAL_0004),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0005, VAL_0001),
            CartNode::init_val_inner(2, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0006, VAL_0002),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0004),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0009, VAL_0006),
            CartNode::init_val_inner(
                0,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0010,
                VAL_0007,
            ),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0011, VAL_0006),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0012, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0016, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(
                0,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0019,
                VAL_0007,
            ),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0020, VAL_0006),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0021, VAL_0008),
            CartNode::init_val_inner(2, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0022, VAL_0002),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0027,
                VAL_0010,
            ),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0028, VAL_0011),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0030, VAL_0008),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0031, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0035, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0037, VAL_0008),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0038, VAL_0008),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0039,
                VAL_0013,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0043,
                VAL_0014,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0045,
                VAL_0015,
            ),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0046,
                VAL_0016,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0049,
                VAL_0017,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0051,
                VAL_0018,
            ),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0052, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0056, VAL_0006),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0057, VAL_0001),
            CartNode::init_val_inner(255, None, 0, VAL_0003),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0059, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0062, VAL_0006),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0063, VAL_0001),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0065, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0067, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0003),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0070, VAL_0019),
            CartNode::init_val_inner(
                0,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0071,
                VAL_0020,
            ),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0072,
                VAL_0021,
            ),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0073,
                VAL_0022,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0003),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0077, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0080, VAL_0023),
            CartNode::init_val_inner(255, None, 0, VAL_0003),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0082, VAL_0008),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_US_NUMS_NO_0083, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(
                0,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0085,
                VAL_0024,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0003),
            CartNode::init_val_inner(255, None, 0, VAL_0009),
            CartNode::init_val_inner(
                0,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0088,
                VAL_0025,
            ),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0089,
                VAL_0026,
            ),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0090,
                VAL_0027,
            ),
            CartNode::init_val_inner(
                6,
                Some(CartOperation::Less),
                CTNODE_US_NUMS_NO_0091,
                VAL_0028,
            ),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0003),
            CartNode::init_val_inner(255, None, 0, VAL_0003),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
        ],
        [
            "num_digits",
            "p.token_pos_guess",
            "month_range",
            "n.token_pos_guess",
            "p.p.token_pos_guess",
            "n.n.token_pos_guess",
            "name",
        ],
    )
}

#[test]
fn test_nums_cart_tree() {
    let x = nums_cart_tree();
}
