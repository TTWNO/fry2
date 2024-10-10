use fry_common::{
    cart_tree::{CartNode, CartOperation, CartTree},
    Value,
};

const VAL_0000: Value<'static> = Value::Str("content");
const VAL_0001: Value<'static> = Value::Str("house");
const VAL_0002: Value<'static> = Value::Str("n");
const VAL_0003: Value<'static> = Value::Str("to");
const VAL_0004: Value<'static> = Value::Str("_other_");
const VAL_0005: Value<'static> = Value::Str("v");
const VAL_0006: Value<'static> = Value::Str("does");
const VAL_0007: Value<'static> = Value::Str("det");
const VAL_0008: Value<'static> = Value::Str("md");
const VAL_0009: Value<'static> = Value::Str("separate");
const VAL_0010: Value<'static> = Value::Str("j");
const VAL_0011: Value<'static> = Value::Str("overall");
const VAL_0012: Value<'static> = Value::Str("perfect");
const VAL_0013: Value<'static> = Value::Str("close");
const VAL_0014: Value<'static> = Value::Str("present");
const VAL_0015: Value<'static> = Value::Str("appropriate");
const VAL_0016: Value<'static> = Value::Str("live");
const VAL_0017: Value<'static> = Value::Str("moderate");
const VAL_0018: Value<'static> = Value::Str("lead");
const VAL_0019: Value<'static> = Value::Str("rebel");
const VAL_0020: Value<'static> = Value::Str("the");
const VAL_0021: Value<'static> = Value::Str("of");
const VAL_0022: Value<'static> = Value::Str("elaborate");
const VAL_0023: Value<'static> = Value::Str("record");
const VAL_0024: Value<'static> = Value::Str("an");
const VAL_0025: Value<'static> = Value::Str("punc");
const VAL_0026: Value<'static> = Value::Str("in");
const VAL_0027: Value<'static> = Value::Str("adv");
const VAL_0028: Value<'static> = Value::Str("nineteen");
const VAL_0029: Value<'static> = Value::Str("six");
const VAL_0030: Value<'static> = Value::Str("putting");
const VAL_0031: Value<'static> = Value::Str("frequent");
const VAL_0032: Value<'static> = Value::Str("bought");
const VAL_0033: Value<'static> = Value::Str("read");
const VAL_0034: Value<'static> = Value::Str("use");
const VAL_0035: Value<'static> = Value::Str("it");
const VAL_0036: Value<'static> = Value::Str("lived");
const VAL_0037: Value<'static> = Value::Str("aged");
const VAL_0038: Value<'static> = Value::Str("uses");
const VAL_0039: Value<'static> = Value::Str("associate");
const VAL_0040: Value<'static> = Value::Str("compact");
const VAL_0041: Value<'static> = Value::Str("suspect");
const VAL_0042: Value<'static> = Value::Str("produce");
const VAL_0043: Value<'static> = Value::Str("produces");
const VAL_0044: Value<'static> = Value::Str("excess");
const VAL_0045: Value<'static> = Value::Str("combine");
const VAL_0046: Value<'static> = Value::Str("tear");
const VAL_0047: Value<'static> = Value::Str("presents");
const VAL_0048: Value<'static> = Value::Str("minute");
const VAL_0049: Value<'static> = Value::Str("rejects");
const VAL_0050: Value<'static> = Value::Str("mrs");
const VAL_0051: Value<'static> = Value::Str("subject");
const VAL_0052: Value<'static> = Value::Str("permits");
const VAL_0053: Value<'static> = Value::Str("reading");
const VAL_0054: Value<'static> = Value::Str("not");
const VAL_0055: Value<'static> = Value::Str("who");
const VAL_0056: Value<'static> = Value::Str("collect");
const VAL_0057: Value<'static> = Value::Str("permit");
const VAL_0058: Value<'static> = Value::Str("that");
const VAL_0059: Value<'static> = Value::Str("records");
const VAL_0060: Value<'static> = Value::Str("survey");
const VAL_0061: Value<'static> = Value::Str("allies");
const VAL_0062: Value<'static> = Value::Str("abuse");
const VAL_0063: Value<'static> = Value::Str("project");
const VAL_0064: Value<'static> = Value::Str("contract");
const VAL_0065: Value<'static> = Value::Str("nasa");
const VAL_0066: Value<'static> = Value::Str("impact");
const VAL_0067: Value<'static> = Value::Str("concert");
const VAL_0068: Value<'static> = Value::Str("progress");
const VAL_0069: Value<'static> = Value::Str("rebels");
const VAL_0070: Value<'static> = Value::Str("associates");
const VAL_0071: Value<'static> = Value::Str("graduate");
const VAL_0072: Value<'static> = Value::Str("export");
const VAL_0073: Value<'static> = Value::Str("increases");
const VAL_0074: Value<'static> = Value::Str("and");
const VAL_0075: Value<'static> = Value::Str("pps");
const VAL_0076: Value<'static> = Value::Str("lives");
const VAL_0077: Value<'static> = Value::Str("sources");
const VAL_0078: Value<'static> = Value::Str("r");
const VAL_0079: Value<'static> = Value::Str("aux");
const VAL_0080: Value<'static> = Value::Str("source");
const VAL_0081: Value<'static> = Value::Str("upset");
const VAL_0082: Value<'static> = Value::Str("object");
const VAL_0083: Value<'static> = Value::Str("at");
const VAL_0084: Value<'static> = Value::Str("refuse");
const VAL_0085: Value<'static> = Value::Str("contrary");
const VAL_0086: Value<'static> = Value::Str("be");
const VAL_0087: Value<'static> = Value::Str("estimate");
const VAL_0088: Value<'static> = Value::Str("estimates");
const VAL_0089: Value<'static> = Value::Str("up");
const VAL_0090: Value<'static> = Value::Str("tears");
const VAL_0091: Value<'static> = Value::Str("0");

const CTNODE_US_POS_NO_0001: usize = 3;
const CTNODE_US_POS_NO_0004: usize = 6;
const CTNODE_US_POS_NO_0003: usize = 7;
const CTNODE_US_POS_NO_0007: usize = 9;
const CTNODE_US_POS_NO_0010: usize = 12;
const CTNODE_US_POS_NO_0009: usize = 13;
const CTNODE_US_POS_NO_0013: usize = 15;
const CTNODE_US_POS_NO_0017: usize = 19;
const CTNODE_US_POS_NO_0019: usize = 21;
const CTNODE_US_POS_NO_0021: usize = 23;
const CTNODE_US_POS_NO_0023: usize = 25;
const CTNODE_US_POS_NO_0025: usize = 27;
const CTNODE_US_POS_NO_0027: usize = 29;
const CTNODE_US_POS_NO_0029: usize = 31;
const CTNODE_US_POS_NO_0031: usize = 33;
const CTNODE_US_POS_NO_0033: usize = 35;
const CTNODE_US_POS_NO_0035: usize = 37;
const CTNODE_US_POS_NO_0038: usize = 40;
const CTNODE_US_POS_NO_0037: usize = 41;
const CTNODE_US_POS_NO_0041: usize = 43;
const CTNODE_US_POS_NO_0043: usize = 45;
const CTNODE_US_POS_NO_0046: usize = 48;
const CTNODE_US_POS_NO_0045: usize = 49;
const CTNODE_US_POS_NO_0050: usize = 52;
const CTNODE_US_POS_NO_0049: usize = 53;
const CTNODE_US_POS_NO_0016: usize = 54;
const CTNODE_US_POS_NO_0055: usize = 57;
const CTNODE_US_POS_NO_0057: usize = 59;
const CTNODE_US_POS_NO_0060: usize = 62;
const CTNODE_US_POS_NO_0062: usize = 64;
const CTNODE_US_POS_NO_0059: usize = 65;
const CTNODE_US_POS_NO_0054: usize = 66;
const CTNODE_US_POS_NO_0066: usize = 68;
const CTNODE_US_POS_NO_0068: usize = 70;
const CTNODE_US_POS_NO_0070: usize = 72;
const CTNODE_US_POS_NO_0072: usize = 74;
const CTNODE_US_POS_NO_0074: usize = 76;
const CTNODE_US_POS_NO_0077: usize = 79;
const CTNODE_US_POS_NO_0079: usize = 81;
const CTNODE_US_POS_NO_0081: usize = 83;
const CTNODE_US_POS_NO_0083: usize = 85;
const CTNODE_US_POS_NO_0076: usize = 86;
const CTNODE_US_POS_NO_0086: usize = 88;
const CTNODE_US_POS_NO_0088: usize = 90;
const CTNODE_US_POS_NO_0090: usize = 92;
const CTNODE_US_POS_NO_0092: usize = 94;
const CTNODE_US_POS_NO_0095: usize = 97;
const CTNODE_US_POS_NO_0097: usize = 99;
const CTNODE_US_POS_NO_0099: usize = 101;
const CTNODE_US_POS_NO_0094: usize = 102;
const CTNODE_US_POS_NO_0102: usize = 104;
const CTNODE_US_POS_NO_0104: usize = 106;
const CTNODE_US_POS_NO_0106: usize = 108;
const CTNODE_US_POS_NO_0108: usize = 110;
const CTNODE_US_POS_NO_0110: usize = 112;
const CTNODE_US_POS_NO_0112: usize = 114;
const CTNODE_US_POS_NO_0114: usize = 116;
const CTNODE_US_POS_NO_0116: usize = 118;
const CTNODE_US_POS_NO_0118: usize = 120;
const CTNODE_US_POS_NO_0120: usize = 122;
const CTNODE_US_POS_NO_0123: usize = 125;
const CTNODE_US_POS_NO_0122: usize = 126;
const CTNODE_US_POS_NO_0126: usize = 128;
const CTNODE_US_POS_NO_0128: usize = 130;
const CTNODE_US_POS_NO_0130: usize = 132;
const CTNODE_US_POS_NO_0132: usize = 134;
const CTNODE_US_POS_NO_0134: usize = 136;
const CTNODE_US_POS_NO_0136: usize = 138;
const CTNODE_US_POS_NO_0138: usize = 140;
const CTNODE_US_POS_NO_0140: usize = 142;
const CTNODE_US_POS_NO_0142: usize = 144;
const CTNODE_US_POS_NO_0144: usize = 146;
const CTNODE_US_POS_NO_0146: usize = 148;
const CTNODE_US_POS_NO_0148: usize = 150;
const CTNODE_US_POS_NO_0152: usize = 154;
const CTNODE_US_POS_NO_0151: usize = 155;
const CTNODE_US_POS_NO_0150: usize = 156;
const CTNODE_US_POS_NO_0156: usize = 158;
const CTNODE_US_POS_NO_0158: usize = 160;
const CTNODE_US_POS_NO_0160: usize = 162;
const CTNODE_US_POS_NO_0162: usize = 164;
const CTNODE_US_POS_NO_0164: usize = 166;
const CTNODE_US_POS_NO_0166: usize = 168;
const CTNODE_US_POS_NO_0169: usize = 171;
const CTNODE_US_POS_NO_0168: usize = 172;
const CTNODE_US_POS_NO_0172: usize = 174;
const CTNODE_US_POS_NO_0174: usize = 176;
const CTNODE_US_POS_NO_0176: usize = 178;
const CTNODE_US_POS_NO_0178: usize = 180;
const CTNODE_US_POS_NO_0180: usize = 182;
const CTNODE_US_POS_NO_0182: usize = 184;
const CTNODE_US_POS_NO_0184: usize = 186;
const CTNODE_US_POS_NO_0186: usize = 188;
const CTNODE_US_POS_NO_0189: usize = 191;
const CTNODE_US_POS_NO_0191: usize = 193;
const CTNODE_US_POS_NO_0193: usize = 195;
const CTNODE_US_POS_NO_0195: usize = 197;
const CTNODE_US_POS_NO_0197: usize = 199;
const CTNODE_US_POS_NO_0199: usize = 201;
const CTNODE_US_POS_NO_0201: usize = 203;
const CTNODE_US_POS_NO_0203: usize = 205;
const CTNODE_US_POS_NO_0205: usize = 207;
const CTNODE_US_POS_NO_0207: usize = 209;
const CTNODE_US_POS_NO_0209: usize = 211;
const CTNODE_US_POS_NO_0211: usize = 213;
const CTNODE_US_POS_NO_0213: usize = 215;
const CTNODE_US_POS_NO_0215: usize = 217;
const CTNODE_US_POS_NO_0217: usize = 219;
const CTNODE_US_POS_NO_0219: usize = 221;
const CTNODE_US_POS_NO_0188: usize = 222;
const CTNODE_US_POS_NO_0222: usize = 224;
const CTNODE_US_POS_NO_0015: usize = 225;
const CTNODE_US_POS_NO_0226: usize = 228;
const CTNODE_US_POS_NO_0228: usize = 230;
const CTNODE_US_POS_NO_0230: usize = 232;
const CTNODE_US_POS_NO_0232: usize = 234;
const CTNODE_US_POS_NO_0234: usize = 236;
const CTNODE_US_POS_NO_0236: usize = 238;
const CTNODE_US_POS_NO_0238: usize = 240;
const CTNODE_US_POS_NO_0241: usize = 243;
const CTNODE_US_POS_NO_0240: usize = 244;
const CTNODE_US_POS_NO_0244: usize = 246;
const CTNODE_US_POS_NO_0246: usize = 248;
const CTNODE_US_POS_NO_0248: usize = 250;
const CTNODE_US_POS_NO_0225: usize = 251;
const CTNODE_US_POS_NO_0252: usize = 254;
const CTNODE_US_POS_NO_0254: usize = 256;
const CTNODE_US_POS_NO_0256: usize = 258;
const CTNODE_US_POS_NO_0258: usize = 260;
const CTNODE_US_POS_NO_0260: usize = 262;
const CTNODE_US_POS_NO_0262: usize = 264;
const CTNODE_US_POS_NO_0264: usize = 266;
const CTNODE_US_POS_NO_0266: usize = 268;
const CTNODE_US_POS_NO_0268: usize = 270;
const CTNODE_US_POS_NO_0270: usize = 272;
const CTNODE_US_POS_NO_0251: usize = 273;
const CTNODE_US_POS_NO_0274: usize = 276;
const CTNODE_US_POS_NO_0273: usize = 277;
const CTNODE_US_POS_NO_0277: usize = 279;
const CTNODE_US_POS_NO_0280: usize = 282;
const CTNODE_US_POS_NO_0279: usize = 283;
const CTNODE_US_POS_NO_0283: usize = 285;
const CTNODE_US_POS_NO_0285: usize = 287;
const CTNODE_US_POS_NO_0287: usize = 289;
const CTNODE_US_POS_NO_0289: usize = 291;
const CTNODE_US_POS_NO_0291: usize = 293;
const CTNODE_US_POS_NO_0293: usize = 295;
const CTNODE_US_POS_NO_0295: usize = 297;
const CTNODE_US_POS_NO_0297: usize = 299;
const CTNODE_US_POS_NO_0299: usize = 301;
const CTNODE_US_POS_NO_0302: usize = 304;
const CTNODE_US_POS_NO_0304: usize = 306;
const CTNODE_US_POS_NO_0301: usize = 307;
const CTNODE_US_POS_NO_0308: usize = 310;
const CTNODE_US_POS_NO_0310: usize = 312;
const CTNODE_US_POS_NO_0312: usize = 314;
const CTNODE_US_POS_NO_0314: usize = 316;
const CTNODE_US_POS_NO_0307: usize = 317;
const CTNODE_US_POS_NO_0317: usize = 319;
const CTNODE_US_POS_NO_0319: usize = 321;
const CTNODE_US_POS_NO_0321: usize = 323;
const CTNODE_US_POS_NO_0323: usize = 325;
const CTNODE_US_POS_NO_0328: usize = 330;
const CTNODE_US_POS_NO_0327: usize = 331;
const CTNODE_US_POS_NO_0326: usize = 332;
const CTNODE_US_POS_NO_0333: usize = 335;
const CTNODE_US_POS_NO_0336: usize = 338;
const CTNODE_US_POS_NO_0335: usize = 339;
const CTNODE_US_POS_NO_0332: usize = 340;
const CTNODE_US_POS_NO_0325: usize = 341;
const CTNODE_US_POS_NO_0341: usize = 343;
const CTNODE_US_POS_NO_0343: usize = 345;
const CTNODE_US_POS_NO_0346: usize = 348;
const CTNODE_US_POS_NO_0349: usize = 351;
const CTNODE_US_POS_NO_0348: usize = 352;
const CTNODE_US_POS_NO_0345: usize = 353;
const CTNODE_US_POS_NO_0353: usize = 355;
const CTNODE_US_POS_NO_0355: usize = 357;
const CTNODE_US_POS_NO_0357: usize = 359;
const CTNODE_US_POS_NO_0359: usize = 361;
const CTNODE_US_POS_NO_0361: usize = 363;
const CTNODE_US_POS_NO_0363: usize = 365;
const CTNODE_US_POS_NO_0000: usize = 366;

static POS_CART_TREE: CartTree<'static, 367, 10> = CartTree::init_unchecked(
    [
        CartNode::init(0, Some(CartOperation::Is), CTNODE_US_POS_NO_0000, VAL_0000),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0001, VAL_0001),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0003, VAL_0003),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0004, VAL_0004),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0007, VAL_0006),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_US_POS_NO_0009, VAL_0007),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0010, VAL_0007),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0013, VAL_0008),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_US_POS_NO_0015, VAL_0000),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0016, VAL_0007),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0017, VAL_0009),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0019, VAL_0011),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0021, VAL_0012),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0023, VAL_0013),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0025, VAL_0014),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0027, VAL_0015),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0029, VAL_0016),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0031, VAL_0017),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0033, VAL_0018),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0035, VAL_0019),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0037, VAL_0020),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_US_POS_NO_0038, VAL_0021),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0041, VAL_0022),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_US_POS_NO_0043, VAL_0021),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0045, VAL_0023),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0046, VAL_0000),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0049, VAL_0024),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_US_POS_NO_0050, VAL_0000),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0054, VAL_0004),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_US_POS_NO_0055, VAL_0025),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0057, VAL_0026),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_US_POS_NO_0059, VAL_0027),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0060, VAL_0028),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_US_POS_NO_0062, VAL_0029),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0066, VAL_0013),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0068, VAL_0009),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0070, VAL_0030),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0072, VAL_0011),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0074, VAL_0017),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0076, VAL_0016),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0077, VAL_0026),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0079, VAL_0004),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_US_POS_NO_0081, VAL_0000),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0083, VAL_0000),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0086, VAL_0031),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0088, VAL_0014),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0090, VAL_0032),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0092, VAL_0033),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0094, VAL_0034),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0095, VAL_0026),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0097, VAL_0035),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0099, VAL_0026),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0102, VAL_0036),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0104, VAL_0037),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0106, VAL_0038),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0108, VAL_0012),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0110, VAL_0018),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0112, VAL_0015),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0114, VAL_0039),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0116, VAL_0040),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0118, VAL_0041),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0120, VAL_0022),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0122, VAL_0019),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0123, VAL_0026),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0126, VAL_0042),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0128, VAL_0043),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0130, VAL_0044),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0132, VAL_0045),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0134, VAL_0046),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0136, VAL_0047),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0138, VAL_0048),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0140, VAL_0049),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0142, VAL_0023),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0144, VAL_0050),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0146, VAL_0051),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0148, VAL_0052),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0150, VAL_0053),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0151, VAL_0000),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0152, VAL_0000),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0156, VAL_0054),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0158, VAL_0055),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0160, VAL_0056),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0162, VAL_0000),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0164, VAL_0057),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0166, VAL_0035),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0168, VAL_0058),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_US_POS_NO_0169, VAL_0004),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0172, VAL_0026),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0174, VAL_0059),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0176, VAL_0060),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0178, VAL_0061),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0180, VAL_0062),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0182, VAL_0063),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0184, VAL_0064),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0186, VAL_0065),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0188, VAL_0000),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0189, VAL_0066),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0191, VAL_0067),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0193, VAL_0068),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0195, VAL_0007),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0197, VAL_0069),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0199, VAL_0070),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0201, VAL_0071),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0203, VAL_0026),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0205, VAL_0072),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0207, VAL_0073),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0209, VAL_0000),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_US_POS_NO_0211, VAL_0074),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0213, VAL_0075),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_US_POS_NO_0215, VAL_0003),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0217, VAL_0004),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_US_POS_NO_0219, VAL_0000),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0222, VAL_0076),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0225, VAL_0013),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0226, VAL_0007),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0228, VAL_0077),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0230, VAL_0079),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0232, VAL_0079),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0234, VAL_0026),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0236, VAL_0080),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_US_POS_NO_0238, VAL_0026),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_US_POS_NO_0240, VAL_0004),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0241, VAL_0004),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(5, Some(CartOperation::Is), CTNODE_US_POS_NO_0244, VAL_0020),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_US_POS_NO_0246, VAL_0004),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0248, VAL_0004),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0251, VAL_0079),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0252, VAL_0036),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0254, VAL_0033),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0256, VAL_0081),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0258, VAL_0032),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0260, VAL_0003),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0262, VAL_0014),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0264, VAL_0015),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0266, VAL_0012),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0268, VAL_0004),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_US_POS_NO_0270, VAL_0000),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0273, VAL_0016),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_US_POS_NO_0274, VAL_0025),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0277, VAL_0036),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0279, VAL_0041),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0280, VAL_0007),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0283, VAL_0011),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_US_POS_NO_0285, VAL_0075),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0287, VAL_0033),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0289, VAL_0082),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0291, VAL_0021),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0293, VAL_0055),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0295, VAL_0017),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0297, VAL_0015),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0299, VAL_0012),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0301, VAL_0014),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0302, VAL_0007),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0304, VAL_0083),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0307, VAL_0003),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0308, VAL_0084),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0310, VAL_0085),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0312, VAL_0051),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0314, VAL_0018),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0317, VAL_0009),
        CartNode::init(255, None, 0, VAL_0010),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0319, VAL_0032),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0321, VAL_0030),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0323, VAL_0081),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0325, VAL_0076),
        CartNode::init(3, Some(CartOperation::Is), CTNODE_US_POS_NO_0326, VAL_0026),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0327, VAL_0000),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0328, VAL_0004),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0332, VAL_0075),
        CartNode::init(8, Some(CartOperation::Is), CTNODE_US_POS_NO_0333, VAL_0021),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0335, VAL_0004),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0336, VAL_0026),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0341, VAL_0086),
        CartNode::init(255, None, 0, VAL_0078),
        CartNode::init(2, Some(CartOperation::Is), CTNODE_US_POS_NO_0343, VAL_0054),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0345, VAL_0058),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0346, VAL_0087),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0348, VAL_0088),
        CartNode::init(7, Some(CartOperation::Is), CTNODE_US_POS_NO_0349, VAL_0000),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(9, Some(CartOperation::Is), CTNODE_US_POS_NO_0353, VAL_0089),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0355, VAL_0042),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(4, Some(CartOperation::Is), CTNODE_US_POS_NO_0357, VAL_0007),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0359, VAL_0079),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(1, Some(CartOperation::Is), CTNODE_US_POS_NO_0361, VAL_0090),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(6, Some(CartOperation::Is), CTNODE_US_POS_NO_0363, VAL_0008),
        CartNode::init(255, None, 0, VAL_0005),
        CartNode::init(255, None, 0, VAL_0002),
        CartNode::init(255, None, 0, VAL_0091),
    ],
    [
        "gpos", "name", "p.name", "n.gpos", "p.gpos", "n.n.name", "p.p.gpos", "n.n.gpos",
        "p.p.name", "n.name",
    ],
);
