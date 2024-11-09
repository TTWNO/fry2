use fry_common::{
    cart_tree::{CartNode, CartOperation, CartTree},
    ValueInner,
};

const CTNODE_NO_0001: usize = 3;
const CTNODE_NO_0003: usize = 5;
const CTNODE_NO_0000: usize = 6;
const CTNODE_NO_0011: usize = 13;
const CTNODE_NO_0010: usize = 14;
const CTNODE_NO_0015: usize = 17;
const CTNODE_NO_0017: usize = 19;
const CTNODE_NO_0014: usize = 20;
const CTNODE_NO_0021: usize = 23;
const CTNODE_NO_0023: usize = 25;
const CTNODE_NO_0020: usize = 26;
const CTNODE_NO_0027: usize = 29;
const CTNODE_NO_0029: usize = 31;
const CTNODE_NO_0026: usize = 32;
const CTNODE_NO_0034: usize = 36;
const CTNODE_NO_0033: usize = 37;
const CTNODE_NO_0032: usize = 38;
const CTNODE_NO_0038: usize = 40;
const CTNODE_NO_0040: usize = 42;
const CTNODE_NO_0009: usize = 43;
const CTNODE_NO_0045: usize = 47;
const CTNODE_NO_0047: usize = 49;
const CTNODE_NO_0044: usize = 50;
const CTNODE_NO_0051: usize = 53;
const CTNODE_NO_0054: usize = 56;
const CTNODE_NO_0057: usize = 59;
const CTNODE_NO_0059: usize = 61;
const CTNODE_NO_0056: usize = 62;
const CTNODE_NO_0063: usize = 65;
const CTNODE_NO_0062: usize = 66;
const CTNODE_NO_0053: usize = 67;
const CTNODE_NO_0050: usize = 68;
const CTNODE_NO_0068: usize = 70;
const CTNODE_NO_0070: usize = 72;
const CTNODE_NO_0073: usize = 75;
const CTNODE_NO_0072: usize = 76;
const CTNODE_NO_0043: usize = 77;
const CTNODE_NO_0079: usize = 81;
const CTNODE_NO_0078: usize = 82;
const CTNODE_NO_0077: usize = 83;
const CTNODE_NO_0084: usize = 86;
const CTNODE_NO_0083: usize = 87;
const CTNODE_NO_0087: usize = 89;
const CTNODE_NO_0089: usize = 91;
const CTNODE_NO_0008: usize = 92;
const CTNODE_NO_0095: usize = 97;
const CTNODE_NO_0094: usize = 98;
const CTNODE_NO_0093: usize = 99;
const CTNODE_NO_0101: usize = 103;
const CTNODE_NO_0104: usize = 106;
const CTNODE_NO_0103: usize = 107;
const CTNODE_NO_0100: usize = 108;
const CTNODE_NO_0110: usize = 112;
const CTNODE_NO_0109: usize = 113;
const CTNODE_NO_0108: usize = 114;
const CTNODE_NO_0099: usize = 115;
const CTNODE_NO_0115: usize = 117;
const CTNODE_NO_0119: usize = 121;
const CTNODE_NO_0118: usize = 122;
const CTNODE_NO_0122: usize = 124;
const CTNODE_NO_0117: usize = 125;
const CTNODE_NO_0126: usize = 128;
const CTNODE_NO_0128: usize = 130;
const CTNODE_NO_0125: usize = 131;
const CTNODE_NO_0131: usize = 133;
const CTNODE_NO_0092: usize = 134;
const CTNODE_NO_0135: usize = 137;
const CTNODE_NO_0137: usize = 139;
const CTNODE_NO_0140: usize = 142;
const CTNODE_NO_0139: usize = 143;
const CTNODE_NO_0134: usize = 144;
const CTNODE_NO_0145: usize = 147;
const CTNODE_NO_0144: usize = 148;
const CTNODE_NO_0149: usize = 151;
const CTNODE_NO_0148: usize = 152;
const CTNODE_NO_0152: usize = 154;
const CTNODE_NO_0155: usize = 157;
const CTNODE_NO_0154: usize = 158;
const CTNODE_NO_0007: usize = 159;
const CTNODE_NO_0162: usize = 164;
const CTNODE_NO_0164: usize = 166;
const CTNODE_NO_0166: usize = 168;
const CTNODE_NO_0161: usize = 169;
const CTNODE_NO_0170: usize = 172;
const CTNODE_NO_0169: usize = 173;
const CTNODE_NO_0174: usize = 176;
const CTNODE_NO_0176: usize = 178;
const CTNODE_NO_0178: usize = 180;
const CTNODE_NO_0181: usize = 183;
const CTNODE_NO_0180: usize = 184;
const CTNODE_NO_0173: usize = 185;
const CTNODE_NO_0187: usize = 189;
const CTNODE_NO_0186: usize = 190;
const CTNODE_NO_0185: usize = 191;
const CTNODE_NO_0191: usize = 193;
const CTNODE_NO_0193: usize = 195;
const CTNODE_NO_0197: usize = 199;
const CTNODE_NO_0196: usize = 200;
const CTNODE_NO_0195: usize = 201;
const CTNODE_NO_0202: usize = 204;
const CTNODE_NO_0201: usize = 205;
const CTNODE_NO_0206: usize = 208;
const CTNODE_NO_0205: usize = 209;
const CTNODE_NO_0209: usize = 211;
const CTNODE_NO_0211: usize = 213;
const CTNODE_NO_0160: usize = 214;
const CTNODE_NO_0214: usize = 216;
const CTNODE_NO_0217: usize = 219;
const CTNODE_NO_0216: usize = 220;
const CTNODE_NO_0220: usize = 222;
const CTNODE_NO_0223: usize = 225;
const CTNODE_NO_0222: usize = 226;
const CTNODE_NO_0226: usize = 228;
const CTNODE_NO_0159: usize = 229;
const CTNODE_NO_0231: usize = 233;
const CTNODE_NO_0234: usize = 236;
const CTNODE_NO_0236: usize = 238;
const CTNODE_NO_0233: usize = 239;
const CTNODE_NO_0240: usize = 242;
const CTNODE_NO_0239: usize = 243;
const CTNODE_NO_0243: usize = 245;
const CTNODE_NO_0245: usize = 247;
const CTNODE_NO_0230: usize = 248;
const CTNODE_NO_0252: usize = 254;
const CTNODE_NO_0254: usize = 256;
const CTNODE_NO_0251: usize = 257;
const CTNODE_NO_0258: usize = 260;
const CTNODE_NO_0257: usize = 261;
const CTNODE_NO_0261: usize = 263;
const CTNODE_NO_0250: usize = 264;
const CTNODE_NO_0266: usize = 268;
const CTNODE_NO_0268: usize = 270;
const CTNODE_NO_0265: usize = 271;
const CTNODE_NO_0272: usize = 274;
const CTNODE_NO_0271: usize = 275;
const CTNODE_NO_0275: usize = 277;
const CTNODE_NO_0264: usize = 278;
const CTNODE_NO_0278: usize = 280;
const CTNODE_NO_0249: usize = 281;
const CTNODE_NO_0281: usize = 283;
const CTNODE_NO_0248: usize = 284;
const CTNODE_NO_0287: usize = 289;
const CTNODE_NO_0286: usize = 290;
const CTNODE_NO_0285: usize = 291;
const CTNODE_NO_0291: usize = 293;
const CTNODE_NO_0284: usize = 294;
const CTNODE_NO_0297: usize = 299;
const CTNODE_NO_0300: usize = 302;
const CTNODE_NO_0299: usize = 303;
const CTNODE_NO_0296: usize = 304;
const CTNODE_NO_0304: usize = 306;
const CTNODE_NO_0295: usize = 307;
const CTNODE_NO_0307: usize = 309;
const CTNODE_NO_0309: usize = 311;
const CTNODE_NO_0313: usize = 315;
const CTNODE_NO_0312: usize = 316;
const CTNODE_NO_0316: usize = 318;
const CTNODE_NO_0311: usize = 319;
const CTNODE_NO_0319: usize = 321;
const CTNODE_NO_0294: usize = 322;
const CTNODE_NO_0323: usize = 325;
const CTNODE_NO_0325: usize = 327;
const CTNODE_NO_0328: usize = 330;
const CTNODE_NO_0327: usize = 331;
const CTNODE_NO_0331: usize = 333;
const CTNODE_NO_0322: usize = 334;
const CTNODE_NO_0335: usize = 337;
const CTNODE_NO_0334: usize = 338;
const CTNODE_NO_0340: usize = 342;
const CTNODE_NO_0339: usize = 343;
const CTNODE_NO_0343: usize = 345;
const CTNODE_NO_0338: usize = 346;
const CTNODE_NO_0347: usize = 349;
const CTNODE_NO_0349: usize = 351;
const CTNODE_NO_0346: usize = 352;
const CTNODE_NO_0353: usize = 355;
const CTNODE_NO_0352: usize = 356;
const CTNODE_NO_0229: usize = 357;
const CTNODE_NO_0359: usize = 361;
const CTNODE_NO_0361: usize = 363;
const CTNODE_NO_0358: usize = 364;
const CTNODE_NO_0365: usize = 367;
const CTNODE_NO_0364: usize = 368;
const CTNODE_NO_0368: usize = 370;
const CTNODE_NO_0357: usize = 371;
const CTNODE_NO_0375: usize = 377;
const CTNODE_NO_0374: usize = 378;
const CTNODE_NO_0373: usize = 379;
const CTNODE_NO_0379: usize = 381;
const CTNODE_NO_0372: usize = 382;
const CTNODE_NO_0383: usize = 385;
const CTNODE_NO_0382: usize = 386;
const CTNODE_NO_0388: usize = 390;
const CTNODE_NO_0391: usize = 393;
const CTNODE_NO_0393: usize = 395;
const CTNODE_NO_0390: usize = 396;
const CTNODE_NO_0397: usize = 399;
const CTNODE_NO_0396: usize = 400;
const CTNODE_NO_0401: usize = 403;
const CTNODE_NO_0400: usize = 404;
const CTNODE_NO_0387: usize = 405;
const CTNODE_NO_0406: usize = 408;
const CTNODE_NO_0405: usize = 409;
const CTNODE_NO_0410: usize = 412;
const CTNODE_NO_0409: usize = 413;
const CTNODE_NO_0413: usize = 415;
const CTNODE_NO_0386: usize = 416;
const CTNODE_NO_0371: usize = 417;
const CTNODE_NO_0417: usize = 419;
const CTNODE_NO_0419: usize = 421;
const CTNODE_NO_0006: usize = 422;
const CTNODE_NO_0427: usize = 429;
const CTNODE_NO_0426: usize = 430;
const CTNODE_NO_0431: usize = 433;
const CTNODE_NO_0430: usize = 434;
const CTNODE_NO_0435: usize = 437;
const CTNODE_NO_0434: usize = 438;
const CTNODE_NO_0425: usize = 439;
const CTNODE_NO_0440: usize = 442;
const CTNODE_NO_0442: usize = 444;
const CTNODE_NO_0445: usize = 447;
const CTNODE_NO_0447: usize = 449;
const CTNODE_NO_0444: usize = 450;
const CTNODE_NO_0450: usize = 452;
const CTNODE_NO_0452: usize = 454;
const CTNODE_NO_0439: usize = 455;
const CTNODE_NO_0457: usize = 459;
const CTNODE_NO_0456: usize = 460;
const CTNODE_NO_0462: usize = 464;
const CTNODE_NO_0461: usize = 465;
const CTNODE_NO_0460: usize = 466;
const CTNODE_NO_0466: usize = 468;
const CTNODE_NO_0468: usize = 470;
const CTNODE_NO_0471: usize = 473;
const CTNODE_NO_0470: usize = 474;
const CTNODE_NO_0474: usize = 476;
const CTNODE_NO_0455: usize = 477;
const CTNODE_NO_0478: usize = 480;
const CTNODE_NO_0477: usize = 481;
const CTNODE_NO_0484: usize = 486;
const CTNODE_NO_0483: usize = 487;
const CTNODE_NO_0487: usize = 489;
const CTNODE_NO_0482: usize = 490;
const CTNODE_NO_0491: usize = 493;
const CTNODE_NO_0493: usize = 495;
const CTNODE_NO_0490: usize = 496;
const CTNODE_NO_0496: usize = 498;
const CTNODE_NO_0481: usize = 499;
const CTNODE_NO_0500: usize = 502;
const CTNODE_NO_0499: usize = 503;
const CTNODE_NO_0504: usize = 506;
const CTNODE_NO_0506: usize = 508;
const CTNODE_NO_0503: usize = 509;
const CTNODE_NO_0424: usize = 510;
const CTNODE_NO_0513: usize = 515;
const CTNODE_NO_0515: usize = 517;
const CTNODE_NO_0512: usize = 518;
const CTNODE_NO_0518: usize = 520;
const CTNODE_NO_0520: usize = 522;
const CTNODE_NO_0522: usize = 524;
const CTNODE_NO_0511: usize = 525;
const CTNODE_NO_0525: usize = 527;
const CTNODE_NO_0510: usize = 528;
const CTNODE_NO_0528: usize = 530;
const CTNODE_NO_0530: usize = 532;
const CTNODE_NO_0423: usize = 533;
const CTNODE_NO_0537: usize = 539;
const CTNODE_NO_0540: usize = 542;
const CTNODE_NO_0542: usize = 544;
const CTNODE_NO_0539: usize = 545;
const CTNODE_NO_0546: usize = 548;
const CTNODE_NO_0548: usize = 550;
const CTNODE_NO_0550: usize = 552;
const CTNODE_NO_0545: usize = 553;
const CTNODE_NO_0553: usize = 555;
const CTNODE_NO_0536: usize = 556;
const CTNODE_NO_0556: usize = 558;
const CTNODE_NO_0558: usize = 560;
const CTNODE_NO_0560: usize = 562;
const CTNODE_NO_0535: usize = 563;
const CTNODE_NO_0564: usize = 566;
const CTNODE_NO_0563: usize = 567;
const CTNODE_NO_0569: usize = 571;
const CTNODE_NO_0568: usize = 572;
const CTNODE_NO_0574: usize = 576;
const CTNODE_NO_0573: usize = 577;
const CTNODE_NO_0577: usize = 579;
const CTNODE_NO_0572: usize = 580;
const CTNODE_NO_0580: usize = 582;
const CTNODE_NO_0567: usize = 583;
const CTNODE_NO_0583: usize = 585;
const CTNODE_NO_0534: usize = 586;
const CTNODE_NO_0587: usize = 589;
const CTNODE_NO_0589: usize = 591;
const CTNODE_NO_0591: usize = 593;
const CTNODE_NO_0586: usize = 594;
const CTNODE_NO_0594: usize = 596;
const CTNODE_NO_0597: usize = 599;
const CTNODE_NO_0596: usize = 600;
const CTNODE_NO_0602: usize = 604;
const CTNODE_NO_0605: usize = 607;
const CTNODE_NO_0607: usize = 609;
const CTNODE_NO_0604: usize = 610;
const CTNODE_NO_0601: usize = 611;
const CTNODE_NO_0611: usize = 613;
const CTNODE_NO_0600: usize = 614;
const CTNODE_NO_0614: usize = 616;
const CTNODE_NO_0617: usize = 619;
const CTNODE_NO_0616: usize = 620;
const CTNODE_NO_0621: usize = 623;
const CTNODE_NO_0620: usize = 624;
const CTNODE_NO_0533: usize = 625;
const CTNODE_NO_0626: usize = 628;
const CTNODE_NO_0628: usize = 630;
const CTNODE_NO_0630: usize = 632;
const CTNODE_NO_0633: usize = 635;
const CTNODE_NO_0632: usize = 636;
const CTNODE_NO_0636: usize = 638;
const CTNODE_NO_0625: usize = 639;
const CTNODE_NO_0642: usize = 644;
const CTNODE_NO_0641: usize = 645;
const CTNODE_NO_0645: usize = 647;
const CTNODE_NO_0640: usize = 648;
const CTNODE_NO_0649: usize = 651;
const CTNODE_NO_0648: usize = 652;
const CTNODE_NO_0639: usize = 653;
const CTNODE_NO_0654: usize = 656;
const CTNODE_NO_0653: usize = 657;
const CTNODE_NO_0658: usize = 660;
const CTNODE_NO_0661: usize = 663;
const CTNODE_NO_0660: usize = 664;
const CTNODE_NO_0664: usize = 666;
const CTNODE_NO_0657: usize = 667;
const CTNODE_NO_0668: usize = 670;
const CTNODE_NO_0667: usize = 671;
const CTNODE_NO_0671: usize = 673;
const CTNODE_NO_0422: usize = 674;
const CTNODE_NO_0678: usize = 680;
const CTNODE_NO_0677: usize = 681;
const CTNODE_NO_0676: usize = 682;
const CTNODE_NO_0683: usize = 685;
const CTNODE_NO_0685: usize = 687;
const CTNODE_NO_0687: usize = 689;
const CTNODE_NO_0689: usize = 691;
const CTNODE_NO_0682: usize = 692;
const CTNODE_NO_0693: usize = 695;
const CTNODE_NO_0692: usize = 696;
const CTNODE_NO_0697: usize = 699;
const CTNODE_NO_0699: usize = 701;
const CTNODE_NO_0701: usize = 703;
const CTNODE_NO_0696: usize = 704;
const CTNODE_NO_0704: usize = 706;
const CTNODE_NO_0707: usize = 709;
const CTNODE_NO_0709: usize = 711;
const CTNODE_NO_0713: usize = 715;
const CTNODE_NO_0712: usize = 716;
const CTNODE_NO_0711: usize = 717;
const CTNODE_NO_0706: usize = 718;
const CTNODE_NO_0719: usize = 721;
const CTNODE_NO_0718: usize = 722;
const CTNODE_NO_0722: usize = 724;
const CTNODE_NO_0675: usize = 725;
const CTNODE_NO_0726: usize = 728;
const CTNODE_NO_0728: usize = 730;
const CTNODE_NO_0734: usize = 736;
const CTNODE_NO_0733: usize = 737;
const CTNODE_NO_0732: usize = 738;
const CTNODE_NO_0731: usize = 739;
const CTNODE_NO_0730: usize = 740;
const CTNODE_NO_0725: usize = 741;
const CTNODE_NO_0741: usize = 743;
const CTNODE_NO_0674: usize = 744;
const CTNODE_NO_0746: usize = 748;
const CTNODE_NO_0745: usize = 749;
const CTNODE_NO_0749: usize = 751;
const CTNODE_NO_0752: usize = 754;
const CTNODE_NO_0754: usize = 756;
const CTNODE_NO_0757: usize = 759;
const CTNODE_NO_0756: usize = 760;
const CTNODE_NO_0751: usize = 761;
const CTNODE_NO_0761: usize = 763;
const CTNODE_NO_0763: usize = 765;
const CTNODE_NO_0765: usize = 767;
const CTNODE_NO_0744: usize = 768;
const CTNODE_NO_0770: usize = 772;
const CTNODE_NO_0769: usize = 773;
const CTNODE_NO_0774: usize = 776;
const CTNODE_NO_0773: usize = 777;
const CTNODE_NO_0768: usize = 778;
const CTNODE_NO_0781: usize = 783;
const CTNODE_NO_0783: usize = 785;
const CTNODE_NO_0780: usize = 786;
const CTNODE_NO_0787: usize = 789;
const CTNODE_NO_0786: usize = 790;
const CTNODE_NO_0791: usize = 793;
const CTNODE_NO_0793: usize = 795;
const CTNODE_NO_0796: usize = 798;
const CTNODE_NO_0795: usize = 799;
const CTNODE_NO_0790: usize = 800;
const CTNODE_NO_0802: usize = 804;
const CTNODE_NO_0804: usize = 806;
const CTNODE_NO_0801: usize = 807;
const CTNODE_NO_0807: usize = 809;
const CTNODE_NO_0800: usize = 810;
const CTNODE_NO_0811: usize = 813;
const CTNODE_NO_0815: usize = 817;
const CTNODE_NO_0814: usize = 818;
const CTNODE_NO_0813: usize = 819;
const CTNODE_NO_0810: usize = 820;
const CTNODE_NO_0821: usize = 823;
const CTNODE_NO_0820: usize = 824;
const CTNODE_NO_0825: usize = 827;
const CTNODE_NO_0824: usize = 828;
const CTNODE_NO_0832: usize = 834;
const CTNODE_NO_0834: usize = 836;
const CTNODE_NO_0837: usize = 839;
const CTNODE_NO_0836: usize = 840;
const CTNODE_NO_0831: usize = 841;
const CTNODE_NO_0841: usize = 843;
const CTNODE_NO_0830: usize = 844;
const CTNODE_NO_0844: usize = 846;
const CTNODE_NO_0846: usize = 848;
const CTNODE_NO_0829: usize = 849;
const CTNODE_NO_0849: usize = 851;
const CTNODE_NO_0851: usize = 853;
const CTNODE_NO_0853: usize = 855;
const CTNODE_NO_0855: usize = 857;
const CTNODE_NO_0828: usize = 858;
const CTNODE_NO_0860: usize = 862;
const CTNODE_NO_0862: usize = 864;
const CTNODE_NO_0859: usize = 865;
const CTNODE_NO_0858: usize = 866;
const CTNODE_NO_0866: usize = 868;
const CTNODE_NO_0779: usize = 869;
const CTNODE_NO_0869: usize = 871;
const CTNODE_NO_0871: usize = 873;
const CTNODE_NO_0778: usize = 874;

const VAL_0000: ValueInner<'static> = ValueInner::Str("pau");
const VAL_0001: ValueInner<'static> = ValueInner::Str("+");
const VAL_0002: ValueInner<'static> = ValueInner::Float(-0.500000);
const VAL_0003: ValueInner<'static> = ValueInner::Str("BB");
const VAL_0004: ValueInner<'static> = ValueInner::Float(2.000000);
const VAL_0005: ValueInner<'static> = ValueInner::Float(0.000000);
const VAL_0006: ValueInner<'static> = ValueInner::Str("0");
const VAL_0007: ValueInner<'static> = ValueInner::Float(1.5);
const VAL_0008: ValueInner<'static> = ValueInner::Str("n");
const VAL_0009: ValueInner<'static> = ValueInner::Str("f");
const VAL_0010: ValueInner<'static> = ValueInner::Float(-0.783163);
const VAL_0011: ValueInner<'static> = ValueInner::Float(-0.222704);
const VAL_0012: ValueInner<'static> = ValueInner::Str("s");
const VAL_0013: ValueInner<'static> = ValueInner::Str("2");
const VAL_0014: ValueInner<'static> = ValueInner::Float(0.764459);
const VAL_0015: ValueInner<'static> = ValueInner::Float(0.700000);
const VAL_0016: ValueInner<'static> = ValueInner::Float(0.567944);
const VAL_0017: ValueInner<'static> = ValueInner::Float(0.053027);
const VAL_0018: ValueInner<'static> = ValueInner::Str("l");
const VAL_0019: ValueInner<'static> = ValueInner::Str("1");
const VAL_0020: ValueInner<'static> = ValueInner::Float(0.766486);
const VAL_0021: ValueInner<'static> = ValueInner::Float(0.279248);
const VAL_0022: ValueInner<'static> = ValueInner::Float(0.056777);
const VAL_0023: ValueInner<'static> = ValueInner::Str("coda");
const VAL_0024: ValueInner<'static> = ValueInner::Float(-0.038356);
const VAL_0025: ValueInner<'static> = ValueInner::Float(-0.545853);
const VAL_0026: ValueInner<'static> = ValueInner::Float(-0.765994);
const VAL_0027: ValueInner<'static> = ValueInner::Str("det");
const VAL_0028: ValueInner<'static> = ValueInner::Float(0.300000);
const VAL_0029: ValueInner<'static> = ValueInner::Float(1.000000);
const VAL_0030: ValueInner<'static> = ValueInner::Float(0.160195);
const VAL_0031: ValueInner<'static> = ValueInner::Float(0.713958);
const VAL_0032: ValueInner<'static> = ValueInner::Float(-0.215472);
const VAL_0033: ValueInner<'static> = ValueInner::Str("r");
const VAL_0034: ValueInner<'static> = ValueInner::Float(0.092772);
const VAL_0035: ValueInner<'static> = ValueInner::Float(0.001328);
const VAL_0036: ValueInner<'static> = ValueInner::Float(-0.334898);
const VAL_0037: ValueInner<'static> = ValueInner::Float(0.500000);
const VAL_0038: ValueInner<'static> = ValueInner::Float(0.200000);
const VAL_0039: ValueInner<'static> = ValueInner::Float(-0.041618);
const VAL_0040: ValueInner<'static> = ValueInner::Float(2.3);
const VAL_0041: ValueInner<'static> = ValueInner::Float(0.262952);
const VAL_0042: ValueInner<'static> = ValueInner::Float(0.594794);
const VAL_0043: ValueInner<'static> = ValueInner::Str("mid");
const VAL_0044: ValueInner<'static> = ValueInner::Float(-0.760444);
const VAL_0045: ValueInner<'static> = ValueInner::Float(6.8);
const VAL_0046: ValueInner<'static> = ValueInner::Str("a");
const VAL_0047: ValueInner<'static> = ValueInner::Float(-0.450449);
const VAL_0048: ValueInner<'static> = ValueInner::Float(1.3);
const VAL_0049: ValueInner<'static> = ValueInner::Float(-0.296956);
const VAL_0050: ValueInner<'static> = ValueInner::Float(2.4);
const VAL_0051: ValueInner<'static> = ValueInner::Float(0.042287);
const VAL_0052: ValueInner<'static> = ValueInner::Float(-0.154465);
const VAL_0053: ValueInner<'static> = ValueInner::Float(3.6);
const VAL_0054: ValueInner<'static> = ValueInner::Float(1.2);
const VAL_0055: ValueInner<'static> = ValueInner::Float(-0.264199);
const VAL_0056: ValueInner<'static> = ValueInner::Float(-0.541738);
const VAL_0057: ValueInner<'static> = ValueInner::Float(-0.166623);
const VAL_0058: ValueInner<'static> = ValueInner::Float(-0.571730);
const VAL_0059: ValueInner<'static> = ValueInner::Str("cc");
const VAL_0060: ValueInner<'static> = ValueInner::Float(0.313289);
const VAL_0061: ValueInner<'static> = ValueInner::Float(0.069582);
const VAL_0062: ValueInner<'static> = ValueInner::Float(2.7);
const VAL_0063: ValueInner<'static> = ValueInner::Float(-0.367088);
const VAL_0064: ValueInner<'static> = ValueInner::Float(-0.194887);
const VAL_0065: ValueInner<'static> = ValueInner::Float(-0.063749);
const VAL_0066: ValueInner<'static> = ValueInner::Float(-0.333421);
const VAL_0067: ValueInner<'static> = ValueInner::Float(-0.165383);
const VAL_0068: ValueInner<'static> = ValueInner::Float(-0.516332);
const VAL_0069: ValueInner<'static> = ValueInner::Float(-0.779112);
const VAL_0070: ValueInner<'static> = ValueInner::Float(-0.337611);
const VAL_0071: ValueInner<'static> = ValueInner::Float(1.4);
const VAL_0072: ValueInner<'static> = ValueInner::Float(-0.745807);
const VAL_0073: ValueInner<'static> = ValueInner::Float(-1.049_07);
const VAL_0074: ValueInner<'static> = ValueInner::Float(-0.914974);
const VAL_0075: ValueInner<'static> = ValueInner::Str("initial");
const VAL_0076: ValueInner<'static> = ValueInner::Float(0.172658);
const VAL_0077: ValueInner<'static> = ValueInner::Float(-0.101423);
const VAL_0078: ValueInner<'static> = ValueInner::Float(-0.360092);
const VAL_0079: ValueInner<'static> = ValueInner::Float(2.9);
const VAL_0080: ValueInner<'static> = ValueInner::Float(1.1);
const VAL_0081: ValueInner<'static> = ValueInner::Float(0.764189);
const VAL_0082: ValueInner<'static> = ValueInner::Float(0.555132);
const VAL_0083: ValueInner<'static> = ValueInner::Float(0.369882);
const VAL_0084: ValueInner<'static> = ValueInner::Float(0.666966);
const VAL_0085: ValueInner<'static> = ValueInner::Float(0.400000);
const VAL_0086: ValueInner<'static> = ValueInner::Float(0.240634);
const VAL_0087: ValueInner<'static> = ValueInner::Float(0.486176);
const VAL_0088: ValueInner<'static> = ValueInner::Float(0.573811);
const VAL_0089: ValueInner<'static> = ValueInner::Float(0.194468);
const VAL_0090: ValueInner<'static> = ValueInner::Float(0.499383);
const VAL_0091: ValueInner<'static> = ValueInner::Float(0.073732);
const VAL_0092: ValueInner<'static> = ValueInner::Float(0.331014);
const VAL_0093: ValueInner<'static> = ValueInner::Float(0.092930);
const VAL_0094: ValueInner<'static> = ValueInner::Float(-0.044327);
const VAL_0095: ValueInner<'static> = ValueInner::Float(0.517681);
const VAL_0096: ValueInner<'static> = ValueInner::Float(0.128316);
const VAL_0097: ValueInner<'static> = ValueInner::Float(0.361383);
const VAL_0098: ValueInner<'static> = ValueInner::Float(0.054136);
const VAL_0099: ValueInner<'static> = ValueInner::Float(0.246742);
const VAL_0100: ValueInner<'static> = ValueInner::Float(0.621547);
const VAL_0101: ValueInner<'static> = ValueInner::Float(0.501679);
const VAL_0102: ValueInner<'static> = ValueInner::Float(3.3);
const VAL_0103: ValueInner<'static> = ValueInner::Float(-0.042049);
const VAL_0104: ValueInner<'static> = ValueInner::Float(0.183226);
const VAL_0105: ValueInner<'static> = ValueInner::Float(0.284799);
const VAL_0106: ValueInner<'static> = ValueInner::Float(-0.820934);
const VAL_0107: ValueInner<'static> = ValueInner::Float(-0.348735);
const VAL_0108: ValueInner<'static> = ValueInner::Float(-0.400920);
const VAL_0109: ValueInner<'static> = ValueInner::Float(-0.639366);
const VAL_0110: ValueInner<'static> = ValueInner::Float(0.364857);
const VAL_0111: ValueInner<'static> = ValueInner::Float(3.4);
const VAL_0112: ValueInner<'static> = ValueInner::Float(-0.007686);
const VAL_0113: ValueInner<'static> = ValueInner::Float(-0.197753);
const VAL_0114: ValueInner<'static> = ValueInner::Float(-0.394632);
const VAL_0115: ValueInner<'static> = ValueInner::Float(0.100000);
const VAL_0116: ValueInner<'static> = ValueInner::Float(0.938841);
const VAL_0117: ValueInner<'static> = ValueInner::Float(-0.079664);
const VAL_0118: ValueInner<'static> = ValueInner::Float(0.480026);
const VAL_0119: ValueInner<'static> = ValueInner::Float(0.127175);
const VAL_0120: ValueInner<'static> = ValueInner::Float(-0.708767);
const VAL_0121: ValueInner<'static> = ValueInner::Float(-0.236212);
const VAL_0122: ValueInner<'static> = ValueInner::Float(-0.273389);
const VAL_0123: ValueInner<'static> = ValueInner::Str("in");
const VAL_0124: ValueInner<'static> = ValueInner::Float(0.058134);
const VAL_0125: ValueInner<'static> = ValueInner::Float(0.721904);
const VAL_0126: ValueInner<'static> = ValueInner::Float(2.2);
const VAL_0127: ValueInner<'static> = ValueInner::Float(0.016121);
const VAL_0128: ValueInner<'static> = ValueInner::Float(0.227372);
const VAL_0129: ValueInner<'static> = ValueInner::Float(0.445569);
const VAL_0130: ValueInner<'static> = ValueInner::Float(-0.120097);
const VAL_0131: ValueInner<'static> = ValueInner::Float(0.219042);
const VAL_0132: ValueInner<'static> = ValueInner::Float(0.321245);
const VAL_0133: ValueInner<'static> = ValueInner::Float(0.134075);
const VAL_0134: ValueInner<'static> = ValueInner::Float(-0.466418);
const VAL_0135: ValueInner<'static> = ValueInner::Float(-0.425925);
const VAL_0136: ValueInner<'static> = ValueInner::Float(-0.542809);
const VAL_0137: ValueInner<'static> = ValueInner::Float(-0.201899);
const VAL_0138: ValueInner<'static> = ValueInner::Float(0.209018);
const VAL_0139: ValueInner<'static> = ValueInner::Float(-0.178136);
const VAL_0140: ValueInner<'static> = ValueInner::Float(-0.235593);
const VAL_0141: ValueInner<'static> = ValueInner::Float(0.126118);
const VAL_0142: ValueInner<'static> = ValueInner::Float(-0.174812);
const VAL_0143: ValueInner<'static> = ValueInner::Str("content");
const VAL_0144: ValueInner<'static> = ValueInner::Float(-0.231509);
const VAL_0145: ValueInner<'static> = ValueInner::Float(-0.536405);
const VAL_0146: ValueInner<'static> = ValueInner::Float(0.163343);
const VAL_0147: ValueInner<'static> = ValueInner::Float(-0.455280);
const VAL_0148: ValueInner<'static> = ValueInner::Float(-0.099803);
const VAL_0149: ValueInner<'static> = ValueInner::Float(-0.930547);
const VAL_0150: ValueInner<'static> = ValueInner::Float(-0.634119);
const VAL_0151: ValueInner<'static> = ValueInner::Float(-0.760176);
const VAL_0152: ValueInner<'static> = ValueInner::Float(-0.121355);
const VAL_0153: ValueInner<'static> = ValueInner::Float(-0.557509);
const VAL_0154: ValueInner<'static> = ValueInner::Float(-0.402734);
const VAL_0155: ValueInner<'static> = ValueInner::Float(-0.988478);
const VAL_0156: ValueInner<'static> = ValueInner::Float(-0.802536);
const VAL_0157: ValueInner<'static> = ValueInner::Float(-0.900628);
const VAL_0158: ValueInner<'static> = ValueInner::Float(-0.768992);
const VAL_0159: ValueInner<'static> = ValueInner::Float(-0.574918);
const VAL_0160: ValueInner<'static> = ValueInner::Float(-0.756359);
const VAL_0161: ValueInner<'static> = ValueInner::Float(-0.808937);
const VAL_0162: ValueInner<'static> = ValueInner::Float(-0.933150);
const VAL_0163: ValueInner<'static> = ValueInner::Float(-0.428493);
const VAL_0164: ValueInner<'static> = ValueInner::Float(0.021107);
const VAL_0165: ValueInner<'static> = ValueInner::Float(-0.254485);
const VAL_0166: ValueInner<'static> = ValueInner::Float(-0.389966);
const VAL_0167: ValueInner<'static> = ValueInner::Float(0.185781);
const VAL_0168: ValueInner<'static> = ValueInner::Float(0.422551);
const VAL_0169: ValueInner<'static> = ValueInner::Float(0.145576);
const VAL_0170: ValueInner<'static> = ValueInner::Float(-0.623190);
const VAL_0171: ValueInner<'static> = ValueInner::Float(-0.317324);
const VAL_0172: ValueInner<'static> = ValueInner::Float(-0.591051);
const VAL_0173: ValueInner<'static> = ValueInner::Float(-0.405607);
const VAL_0174: ValueInner<'static> = ValueInner::Float(-0.313148);
const VAL_0175: ValueInner<'static> = ValueInner::Float(0.159416);
const VAL_0176: ValueInner<'static> = ValueInner::Float(-0.254651);
const VAL_0177: ValueInner<'static> = ValueInner::Float(-0.799896);
const VAL_0178: ValueInner<'static> = ValueInner::Float(-0.551309);
const VAL_0179: ValueInner<'static> = ValueInner::Str("final");
const VAL_0180: ValueInner<'static> = ValueInner::Float(-0.707084);
const VAL_0181: ValueInner<'static> = ValueInner::Float(-0.901874);
const VAL_0182: ValueInner<'static> = ValueInner::Float(0.196466);
const VAL_0183: ValueInner<'static> = ValueInner::Float(0.003824);
const VAL_0184: ValueInner<'static> = ValueInner::Float(-0.128590);
const VAL_0185: ValueInner<'static> = ValueInner::Float(-0.219339);
const VAL_0186: ValueInner<'static> = ValueInner::Float(-0.516734);
const VAL_0187: ValueInner<'static> = ValueInner::Str("single");
const VAL_0188: ValueInner<'static> = ValueInner::Float(0.159445);
const VAL_0189: ValueInner<'static> = ValueInner::Float(3.5);
const VAL_0190: ValueInner<'static> = ValueInner::Float(-0.419103);
const VAL_0191: ValueInner<'static> = ValueInner::Float(-0.092856);
const VAL_0192: ValueInner<'static> = ValueInner::Float(-0.576116);
const VAL_0193: ValueInner<'static> = ValueInner::Str("3");
const VAL_0194: ValueInner<'static> = ValueInner::Float(-0.645830);
const VAL_0195: ValueInner<'static> = ValueInner::Float(-0.466500);
const VAL_0196: ValueInner<'static> = ValueInner::Float(-0.217292);
const VAL_0197: ValueInner<'static> = ValueInner::Float(-0.304382);
const VAL_0198: ValueInner<'static> = ValueInner::Float(-0.572203);
const VAL_0199: ValueInner<'static> = ValueInner::Float(-0.240338);
const VAL_0200: ValueInner<'static> = ValueInner::Float(-0.588171);
const VAL_0201: ValueInner<'static> = ValueInner::Float(-0.957970);
const VAL_0202: ValueInner<'static> = ValueInner::Float(3.9);
const VAL_0203: ValueInner<'static> = ValueInner::Float(-0.959427);
const VAL_0204: ValueInner<'static> = ValueInner::Float(-0.845747);
const VAL_0205: ValueInner<'static> = ValueInner::Float(-0.482247);
const VAL_0206: ValueInner<'static> = ValueInner::Float(-0.632362);
const VAL_0207: ValueInner<'static> = ValueInner::Float(-0.713117);
const VAL_0208: ValueInner<'static> = ValueInner::Float(-0.924308);
const VAL_0209: ValueInner<'static> = ValueInner::Float(-0.891342);
const VAL_0210: ValueInner<'static> = ValueInner::Float(-1.152_52);
const VAL_0211: ValueInner<'static> = ValueInner::Float(-0.599624);
const VAL_0212: ValueInner<'static> = ValueInner::Float(-0.077191);
const VAL_0213: ValueInner<'static> = ValueInner::Float(-1.032_42);
const VAL_0214: ValueInner<'static> = ValueInner::Float(-0.542799);
const VAL_0215: ValueInner<'static> = ValueInner::Float(2.8);
const VAL_0216: ValueInner<'static> = ValueInner::Float(-0.423979);
const VAL_0217: ValueInner<'static> = ValueInner::Float(-0.766379);
const VAL_0218: ValueInner<'static> = ValueInner::Str("to");
const VAL_0219: ValueInner<'static> = ValueInner::Float(-0.792895);
const VAL_0220: ValueInner<'static> = ValueInner::Float(-0.276816);
const VAL_0221: ValueInner<'static> = ValueInner::Float(-0.523721);
const VAL_0222: ValueInner<'static> = ValueInner::Float(-0.488102);
const VAL_0223: ValueInner<'static> = ValueInner::Float(-0.731758);
const VAL_0224: ValueInner<'static> = ValueInner::Float(-0.822229);
const VAL_0225: ValueInner<'static> = ValueInner::Float(1.023_34);
const VAL_0226: ValueInner<'static> = ValueInner::Float(0.536277);
const VAL_0227: ValueInner<'static> = ValueInner::Float(0.138201);
const VAL_0228: ValueInner<'static> = ValueInner::Float(-0.234710);
const VAL_0229: ValueInner<'static> = ValueInner::Float(-0.525292);
const VAL_0230: ValueInner<'static> = ValueInner::Float(0.417485);
const VAL_0231: ValueInner<'static> = ValueInner::Float(-0.078200);
const VAL_0232: ValueInner<'static> = ValueInner::Float(-0.569410);
const VAL_0233: ValueInner<'static> = ValueInner::Float(-0.289362);
const VAL_0234: ValueInner<'static> = ValueInner::Float(-0.092104);
const VAL_0235: ValueInner<'static> = ValueInner::Float(0.139463);
const VAL_0236: ValueInner<'static> = ValueInner::Float(-0.070872);
const VAL_0237: ValueInner<'static> = ValueInner::Float(-0.618971);
const VAL_0238: ValueInner<'static> = ValueInner::Float(-0.840495);
const VAL_0239: ValueInner<'static> = ValueInner::Float(0.009134);
const VAL_0240: ValueInner<'static> = ValueInner::Float(-0.512523);
const VAL_0241: ValueInner<'static> = ValueInner::Float(0.121704);
const VAL_0242: ValueInner<'static> = ValueInner::Float(-0.256370);
const VAL_0243: ValueInner<'static> = ValueInner::Float(3.1);
const VAL_0244: ValueInner<'static> = ValueInner::Float(-0.474522);
const VAL_0245: ValueInner<'static> = ValueInner::Float(-0.247206);
const VAL_0246: ValueInner<'static> = ValueInner::Float(-0.597866);
const VAL_0247: ValueInner<'static> = ValueInner::Float(-0.407765);
const VAL_0248: ValueInner<'static> = ValueInner::Float(-0.741256);
const VAL_0249: ValueInner<'static> = ValueInner::Float(-1.084_26);
const VAL_0250: ValueInner<'static> = ValueInner::Float(-0.397890);
const VAL_0251: ValueInner<'static> = ValueInner::Float(2.6);
const VAL_0252: ValueInner<'static> = ValueInner::Float(-0.666011);
const VAL_0253: ValueInner<'static> = ValueInner::Float(-0.499492);
const VAL_0254: ValueInner<'static> = ValueInner::Float(-0.253186);
const VAL_0255: ValueInner<'static> = ValueInner::Float(-0.372832);
const VAL_0256: ValueInner<'static> = ValueInner::Float(-0.093649);
const VAL_0257: ValueInner<'static> = ValueInner::Float(-0.249982);
const VAL_0258: ValueInner<'static> = ValueInner::Float(3.2);
const VAL_0259: ValueInner<'static> = ValueInner::Float(0.180860);
const VAL_0260: ValueInner<'static> = ValueInner::Float(-0.040291);
const VAL_0261: ValueInner<'static> = ValueInner::Str("4");
const VAL_0262: ValueInner<'static> = ValueInner::Float(1.632_03);
const VAL_0263: ValueInner<'static> = ValueInner::Float(0.994933);
const VAL_0264: ValueInner<'static> = ValueInner::Float(0.214457);
const VAL_0265: ValueInner<'static> = ValueInner::Float(0.730381);
const VAL_0266: ValueInner<'static> = ValueInner::Float(-0.336221);
const VAL_0267: ValueInner<'static> = ValueInner::Float(0.468302);
const VAL_0268: ValueInner<'static> = ValueInner::Float(-0.799121);
const VAL_0269: ValueInner<'static> = ValueInner::Float(0.030061);
const VAL_0270: ValueInner<'static> = ValueInner::Str("d");
const VAL_0271: ValueInner<'static> = ValueInner::Float(1.164_9);
const VAL_0272: ValueInner<'static> = ValueInner::Float(2.266_8);
const VAL_0273: ValueInner<'static> = ValueInner::Float(1.503_75);
const VAL_0274: ValueInner<'static> = ValueInner::Float(2.079_27);
const VAL_0275: ValueInner<'static> = ValueInner::Float(1.102_43);
const VAL_0276: ValueInner<'static> = ValueInner::Float(1.843_2);
const VAL_0277: ValueInner<'static> = ValueInner::Float(1.598_53);
const VAL_0278: ValueInner<'static> = ValueInner::Float(1.129_27);
const VAL_0279: ValueInner<'static> = ValueInner::Float(0.442376);
const VAL_0280: ValueInner<'static> = ValueInner::Float(1.765_08);
const VAL_0281: ValueInner<'static> = ValueInner::Float(0.748600);
const VAL_0282: ValueInner<'static> = ValueInner::Float(2.308_26);
const VAL_0283: ValueInner<'static> = ValueInner::Float(1.699_17);
const VAL_0284: ValueInner<'static> = ValueInner::Float(1.311_28);
const VAL_0285: ValueInner<'static> = ValueInner::Float(0.212421);
const VAL_0286: ValueInner<'static> = ValueInner::Float(0.653094);
const VAL_0287: ValueInner<'static> = ValueInner::Float(1.258_02);
const VAL_0288: ValueInner<'static> = ValueInner::Float(0.777568);
const VAL_0289: ValueInner<'static> = ValueInner::Float(0.163941);
const VAL_0290: ValueInner<'static> = ValueInner::Float(-0.167063);
const VAL_0291: ValueInner<'static> = ValueInner::Float(-0.000859);
const VAL_0292: ValueInner<'static> = ValueInner::Float(0.273433);
const VAL_0293: ValueInner<'static> = ValueInner::Float(1.056_94);
const VAL_0294: ValueInner<'static> = ValueInner::Float(0.244916);
const VAL_0295: ValueInner<'static> = ValueInner::Float(1.211_87);
const VAL_0296: ValueInner<'static> = ValueInner::Float(0.598650);
const VAL_0297: ValueInner<'static> = ValueInner::Float(1.163_4);
const VAL_0298: ValueInner<'static> = ValueInner::Float(0.292935);
const VAL_0299: ValueInner<'static> = ValueInner::Float(0.925740);
const VAL_0300: ValueInner<'static> = ValueInner::Float(1.234_84);
const VAL_0301: ValueInner<'static> = ValueInner::Float(2.020_08);
const VAL_0302: ValueInner<'static> = ValueInner::Float(0.697089);
const VAL_0303: ValueInner<'static> = ValueInner::Float(0.992197);
const VAL_0304: ValueInner<'static> = ValueInner::Float(1.510_93);
const VAL_0305: ValueInner<'static> = ValueInner::Float(0.520952);
const VAL_0306: ValueInner<'static> = ValueInner::Float(0.185827);
const VAL_0307: ValueInner<'static> = ValueInner::Float(0.033230);
const VAL_0308: ValueInner<'static> = ValueInner::Float(-0.534917);
const VAL_0309: ValueInner<'static> = ValueInner::Float(0.575107);
const VAL_0310: ValueInner<'static> = ValueInner::Float(-0.111275);
const VAL_0311: ValueInner<'static> = ValueInner::Float(0.094470);
const VAL_0312: ValueInner<'static> = ValueInner::Float(0.381947);
const VAL_0313: ValueInner<'static> = ValueInner::Float(-0.490108);
const VAL_0314: ValueInner<'static> = ValueInner::Float(-0.201268);
const VAL_0315: ValueInner<'static> = ValueInner::Float(1.203_97);
const VAL_0316: ValueInner<'static> = ValueInner::Float(0.636568);
const VAL_0317: ValueInner<'static> = ValueInner::Float(1.077_63);
const VAL_0318: ValueInner<'static> = ValueInner::Float(-0.016336);
const VAL_0319: ValueInner<'static> = ValueInner::Float(1.072_53);
const VAL_0320: ValueInner<'static> = ValueInner::Float(0.525806);
const VAL_0321: ValueInner<'static> = ValueInner::Float(0.952792);
const VAL_0322: ValueInner<'static> = ValueInner::Float(0.469117);
const VAL_0323: ValueInner<'static> = ValueInner::Float(-0.071645);
const VAL_0324: ValueInner<'static> = ValueInner::Float(0.457137);
const VAL_0325: ValueInner<'static> = ValueInner::Float(0.102492);
const VAL_0326: ValueInner<'static> = ValueInner::Float(0.697337);
const VAL_0327: ValueInner<'static> = ValueInner::Float(0.375114);
const VAL_0328: ValueInner<'static> = ValueInner::Float(0.410671);
const VAL_0329: ValueInner<'static> = ValueInner::Float(0.800000);
const VAL_0330: ValueInner<'static> = ValueInner::Float(-0.331055);
const VAL_0331: ValueInner<'static> = ValueInner::Float(-0.240616);
const VAL_0332: ValueInner<'static> = ValueInner::Float(-0.019127);
const VAL_0333: ValueInner<'static> = ValueInner::Float(0.556537);
const VAL_0334: ValueInner<'static> = ValueInner::Float(0.153892);
const VAL_0335: ValueInner<'static> = ValueInner::Float(0.123242);
const VAL_0336: ValueInner<'static> = ValueInner::Float(0.295753);
const VAL_0337: ValueInner<'static> = ValueInner::Float(-0.341018);
const VAL_0338: ValueInner<'static> = ValueInner::Float(-0.008931);
const VAL_0339: ValueInner<'static> = ValueInner::Float(-0.744625);
const VAL_0340: ValueInner<'static> = ValueInner::Float(-0.302803);
const VAL_0341: ValueInner<'static> = ValueInner::Float(0.113815);
const VAL_0342: ValueInner<'static> = ValueInner::Float(-0.128733);
const VAL_0343: ValueInner<'static> = ValueInner::Float(-0.854509);
const VAL_0344: ValueInner<'static> = ValueInner::Float(-0.216179);
const VAL_0345: ValueInner<'static> = ValueInner::Float(0.461950);
const VAL_0346: ValueInner<'static> = ValueInner::Float(0.657169);
const VAL_0347: ValueInner<'static> = ValueInner::Float(1.082_22);
const VAL_0348: ValueInner<'static> = ValueInner::Float(1.462_57);
const VAL_0349: ValueInner<'static> = ValueInner::Float(0.785204);
const VAL_0350: ValueInner<'static> = ValueInner::Float(0.321168);
const VAL_0351: ValueInner<'static> = ValueInner::Float(0.950834);
const VAL_0352: ValueInner<'static> = ValueInner::Float(-0.167374);
const VAL_0353: ValueInner<'static> = ValueInner::Float(-0.003744);
const VAL_0354: ValueInner<'static> = ValueInner::Float(0.228448);
const VAL_0355: ValueInner<'static> = ValueInner::Float(0.504252);
const VAL_0356: ValueInner<'static> = ValueInner::Float(0.736476);
const VAL_0357: ValueInner<'static> = ValueInner::Float(0.059097);
const VAL_0358: ValueInner<'static> = ValueInner::Float(-0.431535);
const VAL_0359: ValueInner<'static> = ValueInner::Float(1.006_42);
const VAL_0360: ValueInner<'static> = ValueInner::Float(0.481652);
const VAL_0361: ValueInner<'static> = ValueInner::Float(0.749861);
const VAL_0362: ValueInner<'static> = ValueInner::Float(0.069631);
const VAL_0363: ValueInner<'static> = ValueInner::Float(0.552212);
const VAL_0364: ValueInner<'static> = ValueInner::Float(-0.047922);
const VAL_0365: ValueInner<'static> = ValueInner::Float(-1.060_9);
const VAL_0366: ValueInner<'static> = ValueInner::Float(-0.599330);
const VAL_0367: ValueInner<'static> = ValueInner::Float(0.006987);
const VAL_0368: ValueInner<'static> = ValueInner::Float(-0.064904);
const VAL_0369: ValueInner<'static> = ValueInner::Float(-0.248899);
const VAL_0370: ValueInner<'static> = ValueInner::Float(-0.601987);
const VAL_0371: ValueInner<'static> = ValueInner::Float(-0.302401);
const VAL_0372: ValueInner<'static> = ValueInner::Float(0.164636);
const VAL_0373: ValueInner<'static> = ValueInner::Float(-0.098905);
const VAL_0374: ValueInner<'static> = ValueInner::Float(-0.316836);
const VAL_0375: ValueInner<'static> = ValueInner::Float(-0.096119);
const VAL_0376: ValueInner<'static> = ValueInner::Float(-0.429437);
const VAL_0377: ValueInner<'static> = ValueInner::Float(-0.065274);
const VAL_0378: ValueInner<'static> = ValueInner::Float(-0.635089);
const VAL_0379: ValueInner<'static> = ValueInner::Float(0.013936);
const VAL_0380: ValueInner<'static> = ValueInner::Float(-0.454845);
const VAL_0381: ValueInner<'static> = ValueInner::Float(0.809091);
const VAL_0382: ValueInner<'static> = ValueInner::Float(-0.277092);
const VAL_0383: ValueInner<'static> = ValueInner::Float(0.281001);
const VAL_0384: ValueInner<'static> = ValueInner::Float(0.699145);
const VAL_0385: ValueInner<'static> = ValueInner::Float(0.241873);
const VAL_0386: ValueInner<'static> = ValueInner::Float(-0.470784);
const VAL_0387: ValueInner<'static> = ValueInner::Float(-0.072112);
const VAL_0388: ValueInner<'static> = ValueInner::Float(0.073349);
const VAL_0389: ValueInner<'static> = ValueInner::Float(0.608371);
const VAL_0390: ValueInner<'static> = ValueInner::Float(-0.293282);
const VAL_0391: ValueInner<'static> = ValueInner::Float(-0.081611);
const VAL_0392: ValueInner<'static> = ValueInner::Float(-0.504024);
const VAL_0393: ValueInner<'static> = ValueInner::Float(0.983950);
const VAL_0394: ValueInner<'static> = ValueInner::Float(0.634789);
const VAL_0395: ValueInner<'static> = ValueInner::Float(4.4);
const VAL_0396: ValueInner<'static> = ValueInner::Float(0.479029);
const VAL_0397: ValueInner<'static> = ValueInner::Float(0.143214);
const VAL_0398: ValueInner<'static> = ValueInner::Float(0.406834);
const VAL_0399: ValueInner<'static> = ValueInner::Float(0.600000);
const VAL_0400: ValueInner<'static> = ValueInner::Float(-0.415599);
const VAL_0401: ValueInner<'static> = ValueInner::Float(0.110288);
const VAL_0402: ValueInner<'static> = ValueInner::Float(0.031419);
const VAL_0403: ValueInner<'static> = ValueInner::Float(0.693893);
const VAL_0404: ValueInner<'static> = ValueInner::Float(6.000000);
const VAL_0405: ValueInner<'static> = ValueInner::Float(0.215675);
const VAL_0406: ValueInner<'static> = ValueInner::Float(0.574068);
const VAL_0407: ValueInner<'static> = ValueInner::Float(-0.458142);
const VAL_0408: ValueInner<'static> = ValueInner::Float(0.304628);
const VAL_0409: ValueInner<'static> = ValueInner::Float(-0.230940);
const VAL_0410: ValueInner<'static> = ValueInner::Float(0.326954);
const VAL_0411: ValueInner<'static> = ValueInner::Float(-0.100616);
const VAL_0412: ValueInner<'static> = ValueInner::Float(-0.091913);
const VAL_0413: ValueInner<'static> = ValueInner::Float(0.219053);
const VAL_0414: ValueInner<'static> = ValueInner::Float(0.216118);
const VAL_0415: ValueInner<'static> = ValueInner::Float(-0.008341);
const VAL_0416: ValueInner<'static> = ValueInner::Float(0.761763);
const VAL_0417: ValueInner<'static> = ValueInner::Float(0.332721);
const VAL_0418: ValueInner<'static> = ValueInner::Float(0.217178);
const VAL_0419: ValueInner<'static> = ValueInner::Float(1.294_51);
const VAL_0420: ValueInner<'static> = ValueInner::Float(4.000000);
const VAL_0421: ValueInner<'static> = ValueInner::Float(0.945261);
const VAL_0422: ValueInner<'static> = ValueInner::Float(0.687498);
const VAL_0423: ValueInner<'static> = ValueInner::Float(0.403076);
const VAL_0424: ValueInner<'static> = ValueInner::Float(1.002_55);
const VAL_0425: ValueInner<'static> = ValueInner::Float(1.091_13);
const VAL_0426: ValueInner<'static> = ValueInner::Float(0.209045);
const VAL_0427: ValueInner<'static> = ValueInner::Float(-0.054407);
const VAL_0428: ValueInner<'static> = ValueInner::Float(0.256045);
const VAL_0429: ValueInner<'static> = ValueInner::Float(-0.967300);
const VAL_0430: ValueInner<'static> = ValueInner::Float(-0.351397);
const VAL_0431: ValueInner<'static> = ValueInner::Float(-0.623300);
const VAL_0432: ValueInner<'static> = ValueInner::Float(0.266234);
const VAL_0433: ValueInner<'static> = ValueInner::Float(-0.302281);
const VAL_0434: ValueInner<'static> = ValueInner::Float(-0.010244);
const VAL_0435: ValueInner<'static> = ValueInner::Float(-0.274514);
const VAL_0436: ValueInner<'static> = ValueInner::Float(0.048218);
const VAL_0437: ValueInner<'static> = ValueInner::Float(0.888495);
const VAL_0438: ValueInner<'static> = ValueInner::Float(0.653018);
const VAL_0439: ValueInner<'static> = ValueInner::Float(0.061289);
const VAL_0440: ValueInner<'static> = ValueInner::Float(0.346637);
const VAL_0441: ValueInner<'static> = ValueInner::Float(0.041181);
const VAL_0442: ValueInner<'static> = ValueInner::Float(0.613305);
const VAL_0443: ValueInner<'static> = ValueInner::Float(0.175467);
const VAL_0444: ValueInner<'static> = ValueInner::Float(-0.276407);
const VAL_0445: ValueInner<'static> = ValueInner::Float(-0.550878);
const VAL_0446: ValueInner<'static> = ValueInner::Float(-0.240328);
const VAL_0447: ValueInner<'static> = ValueInner::Float(0.330352);
const VAL_0448: ValueInner<'static> = ValueInner::Float(-0.081668);
const VAL_0449: ValueInner<'static> = ValueInner::Float(0.383533);
const VAL_0450: ValueInner<'static> = ValueInner::Float(-0.324515);
const VAL_0451: ValueInner<'static> = ValueInner::Float(-0.624870);
const VAL_0452: ValueInner<'static> = ValueInner::Float(-0.513869);
const VAL_0453: ValueInner<'static> = ValueInner::Float(0.207874);
const VAL_0454: ValueInner<'static> = ValueInner::Float(-0.020471);
const VAL_0455: ValueInner<'static> = ValueInner::Float(0.397372);
const VAL_0456: ValueInner<'static> = ValueInner::Float(0.271734);
const VAL_0457: ValueInner<'static> = ValueInner::Float(-0.261466);
const VAL_0458: ValueInner<'static> = ValueInner::Float(-0.009566);
const VAL_0459: ValueInner<'static> = ValueInner::Float(-0.381895);
const VAL_0460: ValueInner<'static> = ValueInner::Float(-0.089877);
const VAL_0461: ValueInner<'static> = ValueInner::Float(-1.126_26);
const VAL_0462: ValueInner<'static> = ValueInner::Float(-0.906926);
const VAL_0463: ValueInner<'static> = ValueInner::Float(-0.625651);
const VAL_0464: ValueInner<'static> = ValueInner::Float(-0.385089);
const VAL_0465: ValueInner<'static> = ValueInner::Float(-0.359702);
const VAL_0466: ValueInner<'static> = ValueInner::Float(0.216904);
const VAL_0467: ValueInner<'static> = ValueInner::Float(-0.394349);
const VAL_0468: ValueInner<'static> = ValueInner::Float(-0.860573);
const VAL_0469: ValueInner<'static> = ValueInner::Float(-0.510488);
const VAL_0470: ValueInner<'static> = ValueInner::Float(0.010843);
const VAL_0471: ValueInner<'static> = ValueInner::Float(-0.035054);
const VAL_0472: ValueInner<'static> = ValueInner::Float(-0.179727);
const VAL_0473: ValueInner<'static> = ValueInner::Float(-0.297341);
const VAL_0474: ValueInner<'static> = ValueInner::Float(-0.542602);
const VAL_0475: ValueInner<'static> = ValueInner::Float(-0.604960);
const VAL_0476: ValueInner<'static> = ValueInner::Float(-0.432058);
const VAL_0477: ValueInner<'static> = ValueInner::Float(-0.389079);
const VAL_0478: ValueInner<'static> = ValueInner::Float(-0.735640);
const VAL_0479: ValueInner<'static> = ValueInner::Float(-0.605444);
const VAL_0480: ValueInner<'static> = ValueInner::Float(-0.827377);
const VAL_0481: ValueInner<'static> = ValueInner::Float(-0.275338);
const VAL_0482: ValueInner<'static> = ValueInner::Float(-0.802801);
const VAL_0483: ValueInner<'static> = ValueInner::Float(-0.371234);
const VAL_0484: ValueInner<'static> = ValueInner::Float(-0.772883);
const VAL_0485: ValueInner<'static> = ValueInner::Float(-0.655006);
const VAL_0486: ValueInner<'static> = ValueInner::Float(-0.303751);
const VAL_0487: ValueInner<'static> = ValueInner::Float(-0.456882);
const VAL_0488: ValueInner<'static> = ValueInner::Float(-0.133182);
const VAL_0489: ValueInner<'static> = ValueInner::Float(0.114442);
const VAL_0490: ValueInner<'static> = ValueInner::Float(-0.167545);
const VAL_0491: ValueInner<'static> = ValueInner::Float(-0.876950);
const VAL_0492: ValueInner<'static> = ValueInner::Float(-0.640572);
const VAL_0493: ValueInner<'static> = ValueInner::Float(-0.321322);
const VAL_0494: ValueInner<'static> = ValueInner::Float(-0.925472);

fn durz_cart_tree() -> CartTree<'static, 875, 29> {
    CartTree::init_unchecked(
        [
            CartNode::init_val_inner(0, Some(CartOperation::Is), CTNODE_NO_0000, VAL_0000),
            CartNode::init_val_inner(1, Some(CartOperation::Is), CTNODE_NO_0001, VAL_0001),
            CartNode::init_val_inner(255, None, 0, VAL_0002),
            CartNode::init_val_inner(2, Some(CartOperation::Is), CTNODE_NO_0003, VAL_0003),
            CartNode::init_val_inner(255, None, 0, VAL_0004),
            CartNode::init_val_inner(255, None, 0, VAL_0005),
            CartNode::init_val_inner(3, Some(CartOperation::Is), CTNODE_NO_0006, VAL_0006),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0007, VAL_0006),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0008, VAL_0006),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0009, VAL_0007),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0010, VAL_0008),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0011, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0010),
            CartNode::init_val_inner(255, None, 0, VAL_0011),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0014, VAL_0012),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0015, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0014),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0017, VAL_0015),
            CartNode::init_val_inner(255, None, 0, VAL_0016),
            CartNode::init_val_inner(255, None, 0, VAL_0017),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0020, VAL_0018),
            CartNode::init_val_inner(12, Some(CartOperation::Is), CTNODE_NO_0021, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0020),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0023, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0021),
            CartNode::init_val_inner(255, None, 0, VAL_0022),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0026, VAL_0012),
            CartNode::init_val_inner(13, Some(CartOperation::Is), CTNODE_NO_0027, VAL_0023),
            CartNode::init_val_inner(255, None, 0, VAL_0024),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0029, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0025),
            CartNode::init_val_inner(255, None, 0, VAL_0026),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0032, VAL_0027),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0033, VAL_0028),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0034, VAL_0029),
            CartNode::init_val_inner(255, None, 0, VAL_0030),
            CartNode::init_val_inner(255, None, 0, VAL_0031),
            CartNode::init_val_inner(255, None, 0, VAL_0032),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0038, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0034),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0040, VAL_0029),
            CartNode::init_val_inner(255, None, 0, VAL_0035),
            CartNode::init_val_inner(255, None, 0, VAL_0036),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0043, VAL_0037),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0044, VAL_0013),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0045, VAL_0038),
            CartNode::init_val_inner(255, None, 0, VAL_0039),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0047, VAL_0040),
            CartNode::init_val_inner(255, None, 0, VAL_0041),
            CartNode::init_val_inner(255, None, 0, VAL_0042),
            CartNode::init_val_inner(20, Some(CartOperation::Is), CTNODE_NO_0050, VAL_0006),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0051, VAL_0043),
            CartNode::init_val_inner(255, None, 0, VAL_0044),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0053, VAL_0045),
            CartNode::init_val_inner(23, Some(CartOperation::Is), CTNODE_NO_0054, VAL_0046),
            CartNode::init_val_inner(255, None, 0, VAL_0047),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0056, VAL_0009),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0057, VAL_0048),
            CartNode::init_val_inner(255, None, 0, VAL_0049),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0059, VAL_0050),
            CartNode::init_val_inner(255, None, 0, VAL_0051),
            CartNode::init_val_inner(255, None, 0, VAL_0052),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0062, VAL_0053),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0063, VAL_0054),
            CartNode::init_val_inner(255, None, 0, VAL_0055),
            CartNode::init_val_inner(255, None, 0, VAL_0056),
            CartNode::init_val_inner(255, None, 0, VAL_0057),
            CartNode::init_val_inner(255, None, 0, VAL_0058),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0068, VAL_0059),
            CartNode::init_val_inner(255, None, 0, VAL_0060),
            CartNode::init_val_inner(12, Some(CartOperation::Is), CTNODE_NO_0070, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0061),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0072, VAL_0054),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0073, VAL_0062),
            CartNode::init_val_inner(255, None, 0, VAL_0063),
            CartNode::init_val_inner(255, None, 0, VAL_0064),
            CartNode::init_val_inner(255, None, 0, VAL_0065),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0077, VAL_0012),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0078, VAL_0006),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0079, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0066),
            CartNode::init_val_inner(255, None, 0, VAL_0067),
            CartNode::init_val_inner(255, None, 0, VAL_0068),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0083, VAL_0006),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0084, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0069),
            CartNode::init_val_inner(255, None, 0, VAL_0070),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0087, VAL_0071),
            CartNode::init_val_inner(255, None, 0, VAL_0072),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0089, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0073),
            CartNode::init_val_inner(255, None, 0, VAL_0074),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0092, VAL_0006),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0093, VAL_0008),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0094, VAL_0075),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0095, VAL_0054),
            CartNode::init_val_inner(255, None, 0, VAL_0076),
            CartNode::init_val_inner(255, None, 0, VAL_0077),
            CartNode::init_val_inner(255, None, 0, VAL_0078),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0099, VAL_0079),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0100, VAL_0080),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0101, VAL_0075),
            CartNode::init_val_inner(255, None, 0, VAL_0081),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0103, VAL_0053),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0104, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0082),
            CartNode::init_val_inner(255, None, 0, VAL_0083),
            CartNode::init_val_inner(255, None, 0, VAL_0084),
            CartNode::init_val_inner(13, Some(CartOperation::Is), CTNODE_NO_0108, VAL_0023),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0109, VAL_0046),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0110, VAL_0085),
            CartNode::init_val_inner(255, None, 0, VAL_0086),
            CartNode::init_val_inner(255, None, 0, VAL_0087),
            CartNode::init_val_inner(255, None, 0, VAL_0088),
            CartNode::init_val_inner(255, None, 0, VAL_0089),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0115, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0090),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0117, VAL_0037),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0118, VAL_0075),
            CartNode::init_val_inner(24, Some(CartOperation::Less), CTNODE_NO_0119, VAL_0050),
            CartNode::init_val_inner(255, None, 0, VAL_0091),
            CartNode::init_val_inner(255, None, 0, VAL_0092),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0122, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0093),
            CartNode::init_val_inner(255, None, 0, VAL_0094),
            CartNode::init_val_inner(25, Some(CartOperation::Is), CTNODE_NO_0125, VAL_0006),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0126, VAL_0046),
            CartNode::init_val_inner(255, None, 0, VAL_0095),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0128, VAL_0080),
            CartNode::init_val_inner(255, None, 0, VAL_0096),
            CartNode::init_val_inner(255, None, 0, VAL_0097),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0131, VAL_0048),
            CartNode::init_val_inner(255, None, 0, VAL_0098),
            CartNode::init_val_inner(255, None, 0, VAL_0099),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0134, VAL_0028),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0135, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0100),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0137, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0101),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0139, VAL_0102),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0140, VAL_0028),
            CartNode::init_val_inner(255, None, 0, VAL_0103),
            CartNode::init_val_inner(255, None, 0, VAL_0104),
            CartNode::init_val_inner(255, None, 0, VAL_0105),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0144, VAL_0012),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0145, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0106),
            CartNode::init_val_inner(255, None, 0, VAL_0107),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0148, VAL_0008),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0149, VAL_0054),
            CartNode::init_val_inner(255, None, 0, VAL_0108),
            CartNode::init_val_inner(255, None, 0, VAL_0109),
            CartNode::init_val_inner(25, Some(CartOperation::Is), CTNODE_NO_0152, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0110),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0154, VAL_0054),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0155, VAL_0111),
            CartNode::init_val_inner(255, None, 0, VAL_0112),
            CartNode::init_val_inner(255, None, 0, VAL_0113),
            CartNode::init_val_inner(255, None, 0, VAL_0114),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0159, VAL_0009),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0160, VAL_0007),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0161, VAL_0013),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0162, VAL_0115),
            CartNode::init_val_inner(255, None, 0, VAL_0116),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0164, VAL_0075),
            CartNode::init_val_inner(255, None, 0, VAL_0117),
            CartNode::init_val_inner(26, Some(CartOperation::Is), CTNODE_NO_0166, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0118),
            CartNode::init_val_inner(255, None, 0, VAL_0119),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0169, VAL_0033),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0170, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0120),
            CartNode::init_val_inner(255, None, 0, VAL_0121),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0173, VAL_0046),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0174, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0122),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0176, VAL_0123),
            CartNode::init_val_inner(255, None, 0, VAL_0124),
            CartNode::init_val_inner(26, Some(CartOperation::Is), CTNODE_NO_0178, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0125),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0180, VAL_0126),
            CartNode::init_val_inner(12, Some(CartOperation::Is), CTNODE_NO_0181, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0127),
            CartNode::init_val_inner(255, None, 0, VAL_0128),
            CartNode::init_val_inner(255, None, 0, VAL_0129),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0185, VAL_0008),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0186, VAL_0019),
            CartNode::init_val_inner(27, Some(CartOperation::Is), CTNODE_NO_0187, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0130),
            CartNode::init_val_inner(255, None, 0, VAL_0131),
            CartNode::init_val_inner(255, None, 0, VAL_0132),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0191, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0133),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0193, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0134),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0195, VAL_0048),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0196, VAL_0126),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0197, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0135),
            CartNode::init_val_inner(255, None, 0, VAL_0136),
            CartNode::init_val_inner(255, None, 0, VAL_0137),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0201, VAL_0019),
            CartNode::init_val_inner(26, Some(CartOperation::Is), CTNODE_NO_0202, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0138),
            CartNode::init_val_inner(255, None, 0, VAL_0139),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0205, VAL_0028),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0206, VAL_0007),
            CartNode::init_val_inner(255, None, 0, VAL_0140),
            CartNode::init_val_inner(255, None, 0, VAL_0141),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0209, VAL_0046),
            CartNode::init_val_inner(255, None, 0, VAL_0142),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0211, VAL_0143),
            CartNode::init_val_inner(255, None, 0, VAL_0144),
            CartNode::init_val_inner(255, None, 0, VAL_0145),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0214, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0146),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0216, VAL_0012),
            CartNode::init_val_inner(25, Some(CartOperation::Is), CTNODE_NO_0217, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0147),
            CartNode::init_val_inner(255, None, 0, VAL_0148),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0220, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0149),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0222, VAL_0012),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0223, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0150),
            CartNode::init_val_inner(255, None, 0, VAL_0151),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0226, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0152),
            CartNode::init_val_inner(255, None, 0, VAL_0153),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0229, VAL_0006),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0230, VAL_0033),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0231, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0154),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0233, VAL_0007),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0234, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0155),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0236, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0156),
            CartNode::init_val_inner(255, None, 0, VAL_0157),
            CartNode::init_val_inner(26, Some(CartOperation::Is), CTNODE_NO_0239, VAL_0006),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0240, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0158),
            CartNode::init_val_inner(255, None, 0, VAL_0159),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0243, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0160),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0245, VAL_0028),
            CartNode::init_val_inner(255, None, 0, VAL_0161),
            CartNode::init_val_inner(255, None, 0, VAL_0162),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0248, VAL_0046),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0249, VAL_0006),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0250, VAL_0012),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0251, VAL_0009),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0252, VAL_0038),
            CartNode::init_val_inner(255, None, 0, VAL_0163),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0254, VAL_0115),
            CartNode::init_val_inner(255, None, 0, VAL_0164),
            CartNode::init_val_inner(255, None, 0, VAL_0165),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0257, VAL_0012),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0258, VAL_0038),
            CartNode::init_val_inner(255, None, 0, VAL_0166),
            CartNode::init_val_inner(255, None, 0, VAL_0167),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0261, VAL_0071),
            CartNode::init_val_inner(255, None, 0, VAL_0168),
            CartNode::init_val_inner(255, None, 0, VAL_0169),
            CartNode::init_val_inner(20, Some(CartOperation::Is), CTNODE_NO_0264, VAL_0006),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0265, VAL_0006),
            CartNode::init_val_inner(12, Some(CartOperation::Is), CTNODE_NO_0266, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0170),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0268, VAL_0053),
            CartNode::init_val_inner(255, None, 0, VAL_0171),
            CartNode::init_val_inner(255, None, 0, VAL_0172),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0271, VAL_0019),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0272, VAL_0062),
            CartNode::init_val_inner(255, None, 0, VAL_0173),
            CartNode::init_val_inner(255, None, 0, VAL_0174),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0275, VAL_0028),
            CartNode::init_val_inner(255, None, 0, VAL_0175),
            CartNode::init_val_inner(255, None, 0, VAL_0176),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0278, VAL_0075),
            CartNode::init_val_inner(255, None, 0, VAL_0177),
            CartNode::init_val_inner(255, None, 0, VAL_0178),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0281, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0180),
            CartNode::init_val_inner(255, None, 0, VAL_0181),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0284, VAL_0012),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0285, VAL_0054),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0286, VAL_0038),
            CartNode::init_val_inner(12, Some(CartOperation::Is), CTNODE_NO_0287, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0182),
            CartNode::init_val_inner(255, None, 0, VAL_0183),
            CartNode::init_val_inner(255, None, 0, VAL_0184),
            CartNode::init_val_inner(12, Some(CartOperation::Is), CTNODE_NO_0291, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0185),
            CartNode::init_val_inner(255, None, 0, VAL_0186),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0294, VAL_0012),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0295, VAL_0012),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0296, VAL_0143),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0297, VAL_0187),
            CartNode::init_val_inner(255, None, 0, VAL_0188),
            CartNode::init_val_inner(24, Some(CartOperation::Less), CTNODE_NO_0299, VAL_0189),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0300, VAL_0004),
            CartNode::init_val_inner(255, None, 0, VAL_0190),
            CartNode::init_val_inner(255, None, 0, VAL_0191),
            CartNode::init_val_inner(255, None, 0, VAL_0192),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0304, VAL_0193),
            CartNode::init_val_inner(255, None, 0, VAL_0194),
            CartNode::init_val_inner(255, None, 0, VAL_0195),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0307, VAL_0123),
            CartNode::init_val_inner(255, None, 0, VAL_0196),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0309, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0197),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0311, VAL_0143),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0312, VAL_0019),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0313, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0198),
            CartNode::init_val_inner(255, None, 0, VAL_0199),
            CartNode::init_val_inner(24, Some(CartOperation::Less), CTNODE_NO_0316, VAL_0126),
            CartNode::init_val_inner(255, None, 0, VAL_0200),
            CartNode::init_val_inner(255, None, 0, VAL_0201),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0319, VAL_0202),
            CartNode::init_val_inner(255, None, 0, VAL_0203),
            CartNode::init_val_inner(255, None, 0, VAL_0204),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0322, VAL_0006),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0323, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0205),
            CartNode::init_val_inner(26, Some(CartOperation::Is), CTNODE_NO_0325, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0206),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0327, VAL_0018),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0328, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0207),
            CartNode::init_val_inner(255, None, 0, VAL_0208),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0331, VAL_0126),
            CartNode::init_val_inner(255, None, 0, VAL_0209),
            CartNode::init_val_inner(255, None, 0, VAL_0210),
            CartNode::init_val_inner(23, Some(CartOperation::Is), CTNODE_NO_0334, VAL_0012),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0335, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0211),
            CartNode::init_val_inner(255, None, 0, VAL_0212),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0338, VAL_0009),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0339, VAL_0123),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0340, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0213),
            CartNode::init_val_inner(255, None, 0, VAL_0214),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0343, VAL_0215),
            CartNode::init_val_inner(255, None, 0, VAL_0216),
            CartNode::init_val_inner(255, None, 0, VAL_0217),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0346, VAL_0019),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0347, VAL_0218),
            CartNode::init_val_inner(255, None, 0, VAL_0219),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0349, VAL_0018),
            CartNode::init_val_inner(255, None, 0, VAL_0220),
            CartNode::init_val_inner(255, None, 0, VAL_0221),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0352, VAL_0053),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0353, VAL_0075),
            CartNode::init_val_inner(255, None, 0, VAL_0222),
            CartNode::init_val_inner(255, None, 0, VAL_0223),
            CartNode::init_val_inner(255, None, 0, VAL_0224),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0357, VAL_0033),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0358, VAL_0075),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0359, VAL_0046),
            CartNode::init_val_inner(255, None, 0, VAL_0225),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0361, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0226),
            CartNode::init_val_inner(255, None, 0, VAL_0227),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0364, VAL_0080),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0365, VAL_0102),
            CartNode::init_val_inner(255, None, 0, VAL_0228),
            CartNode::init_val_inner(255, None, 0, VAL_0229),
            CartNode::init_val_inner(27, Some(CartOperation::Is), CTNODE_NO_0368, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0230),
            CartNode::init_val_inner(255, None, 0, VAL_0231),
            CartNode::init_val_inner(12, Some(CartOperation::Is), CTNODE_NO_0371, VAL_0006),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0372, VAL_0009),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0373, VAL_0012),
            CartNode::init_val_inner(14, Some(CartOperation::Is), CTNODE_NO_0374, VAL_0143),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0375, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0232),
            CartNode::init_val_inner(255, None, 0, VAL_0233),
            CartNode::init_val_inner(255, None, 0, VAL_0234),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0379, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0235),
            CartNode::init_val_inner(255, None, 0, VAL_0236),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0382, VAL_0012),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0383, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0237),
            CartNode::init_val_inner(255, None, 0, VAL_0238),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0386, VAL_0054),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0387, VAL_0054),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0388, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0239),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0390, VAL_0019),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0391, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0240),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0393, VAL_0075),
            CartNode::init_val_inner(255, None, 0, VAL_0241),
            CartNode::init_val_inner(255, None, 0, VAL_0242),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0396, VAL_0243),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0397, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0244),
            CartNode::init_val_inner(255, None, 0, VAL_0245),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0400, VAL_0126),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0401, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0246),
            CartNode::init_val_inner(255, None, 0, VAL_0247),
            CartNode::init_val_inner(255, None, 0, VAL_0248),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0405, VAL_0012),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0406, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0249),
            CartNode::init_val_inner(255, None, 0, VAL_0250),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0409, VAL_0019),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0410, VAL_0251),
            CartNode::init_val_inner(255, None, 0, VAL_0252),
            CartNode::init_val_inner(255, None, 0, VAL_0253),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0413, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0254),
            CartNode::init_val_inner(255, None, 0, VAL_0255),
            CartNode::init_val_inner(255, None, 0, VAL_0256),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0417, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0257),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0419, VAL_0258),
            CartNode::init_val_inner(255, None, 0, VAL_0259),
            CartNode::init_val_inner(255, None, 0, VAL_0260),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0422, VAL_0050),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0423, VAL_0038),
            CartNode::init_val_inner(13, Some(CartOperation::Is), CTNODE_NO_0424, VAL_0023),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0425, VAL_0012),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0426, VAL_0261),
            CartNode::init_val_inner(23, Some(CartOperation::Is), CTNODE_NO_0427, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0262),
            CartNode::init_val_inner(255, None, 0, VAL_0263),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0430, VAL_0006),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0431, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0264),
            CartNode::init_val_inner(255, None, 0, VAL_0265),
            CartNode::init_val_inner(12, Some(CartOperation::Is), CTNODE_NO_0434, VAL_0006),
            CartNode::init_val_inner(25, Some(CartOperation::Is), CTNODE_NO_0435, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0266),
            CartNode::init_val_inner(255, None, 0, VAL_0267),
            CartNode::init_val_inner(255, None, 0, VAL_0268),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0439, VAL_0009),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0440, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0269),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0442, VAL_0270),
            CartNode::init_val_inner(255, None, 0, VAL_0271),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0444, VAL_0006),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0445, VAL_0054),
            CartNode::init_val_inner(255, None, 0, VAL_0272),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0447, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0273),
            CartNode::init_val_inner(255, None, 0, VAL_0274),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0450, VAL_0080),
            CartNode::init_val_inner(255, None, 0, VAL_0275),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0452, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0276),
            CartNode::init_val_inner(255, None, 0, VAL_0277),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0455, VAL_0006),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0456, VAL_0008),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0457, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0278),
            CartNode::init_val_inner(255, None, 0, VAL_0279),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0460, VAL_0261),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0461, VAL_0179),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0462, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0280),
            CartNode::init_val_inner(255, None, 0, VAL_0281),
            CartNode::init_val_inner(255, None, 0, VAL_0282),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0466, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0283),
            CartNode::init_val_inner(27, Some(CartOperation::Is), CTNODE_NO_0468, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0284),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0470, VAL_0038),
            CartNode::init_val_inner(26, Some(CartOperation::Is), CTNODE_NO_0471, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0285),
            CartNode::init_val_inner(255, None, 0, VAL_0286),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0474, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0287),
            CartNode::init_val_inner(255, None, 0, VAL_0288),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0477, VAL_0009),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0478, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0289),
            CartNode::init_val_inner(255, None, 0, VAL_0290),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0481, VAL_0179),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0482, VAL_0008),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0483, VAL_0037),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0484, VAL_0215),
            CartNode::init_val_inner(255, None, 0, VAL_0291),
            CartNode::init_val_inner(255, None, 0, VAL_0292),
            CartNode::init_val_inner(24, Some(CartOperation::Less), CTNODE_NO_0487, VAL_0050),
            CartNode::init_val_inner(255, None, 0, VAL_0293),
            CartNode::init_val_inner(255, None, 0, VAL_0294),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0490, VAL_0126),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0491, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0295),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0493, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0296),
            CartNode::init_val_inner(255, None, 0, VAL_0297),
            CartNode::init_val_inner(25, Some(CartOperation::Is), CTNODE_NO_0496, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0298),
            CartNode::init_val_inner(255, None, 0, VAL_0299),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0499, VAL_0012),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0500, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0300),
            CartNode::init_val_inner(255, None, 0, VAL_0301),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0503, VAL_0006),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0504, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0302),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0506, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0303),
            CartNode::init_val_inner(255, None, 0, VAL_0304),
            CartNode::init_val_inner(255, None, 0, VAL_0305),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0510, VAL_0006),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0511, VAL_0015),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0512, VAL_0179),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0513, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0306),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0515, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0307),
            CartNode::init_val_inner(255, None, 0, VAL_0308),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0518, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0309),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0520, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0310),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0522, VAL_0028),
            CartNode::init_val_inner(255, None, 0, VAL_0311),
            CartNode::init_val_inner(255, None, 0, VAL_0312),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0525, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0313),
            CartNode::init_val_inner(255, None, 0, VAL_0314),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0528, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0315),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0530, VAL_0054),
            CartNode::init_val_inner(255, None, 0, VAL_0316),
            CartNode::init_val_inner(255, None, 0, VAL_0317),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0533, VAL_0048),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0534, VAL_0006),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0535, VAL_0115),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0536, VAL_0075),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0537, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0318),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0539, VAL_0012),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0540, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0319),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0542, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0320),
            CartNode::init_val_inner(255, None, 0, VAL_0321),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0545, VAL_0006),
            CartNode::init_val_inner(13, Some(CartOperation::Is), CTNODE_NO_0546, VAL_0023),
            CartNode::init_val_inner(255, None, 0, VAL_0322),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0548, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0323),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0550, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0324),
            CartNode::init_val_inner(255, None, 0, VAL_0325),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0553, VAL_0080),
            CartNode::init_val_inner(255, None, 0, VAL_0326),
            CartNode::init_val_inner(255, None, 0, VAL_0327),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0556, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0328),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0558, VAL_0329),
            CartNode::init_val_inner(255, None, 0, VAL_0330),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0560, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0331),
            CartNode::init_val_inner(255, None, 0, VAL_0332),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0563, VAL_0193),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0564, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0333),
            CartNode::init_val_inner(255, None, 0, VAL_0334),
            CartNode::init_val_inner(13, Some(CartOperation::Is), CTNODE_NO_0567, VAL_0023),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0568, VAL_0006),
            CartNode::init_val_inner(24, Some(CartOperation::Less), CTNODE_NO_0569, VAL_0111),
            CartNode::init_val_inner(255, None, 0, VAL_0335),
            CartNode::init_val_inner(255, None, 0, VAL_0336),
            CartNode::init_val_inner(25, Some(CartOperation::Is), CTNODE_NO_0572, VAL_0006),
            CartNode::init_val_inner(24, Some(CartOperation::Less), CTNODE_NO_0573, VAL_0050),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0574, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0337),
            CartNode::init_val_inner(255, None, 0, VAL_0338),
            CartNode::init_val_inner(27, Some(CartOperation::Is), CTNODE_NO_0577, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0339),
            CartNode::init_val_inner(255, None, 0, VAL_0340),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0580, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0341),
            CartNode::init_val_inner(255, None, 0, VAL_0342),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0583, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0343),
            CartNode::init_val_inner(255, None, 0, VAL_0344),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0586, VAL_0009),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0587, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0345),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0589, VAL_0048),
            CartNode::init_val_inner(255, None, 0, VAL_0346),
            CartNode::init_val_inner(23, Some(CartOperation::Is), CTNODE_NO_0591, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0347),
            CartNode::init_val_inner(255, None, 0, VAL_0348),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0594, VAL_0018),
            CartNode::init_val_inner(255, None, 0, VAL_0349),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0596, VAL_0046),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0597, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0350),
            CartNode::init_val_inner(255, None, 0, VAL_0351),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0600, VAL_0006),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0601, VAL_0115),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0602, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0352),
            CartNode::init_val_inner(26, Some(CartOperation::Is), CTNODE_NO_0604, VAL_0006),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0605, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0353),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0607, VAL_0080),
            CartNode::init_val_inner(255, None, 0, VAL_0354),
            CartNode::init_val_inner(255, None, 0, VAL_0355),
            CartNode::init_val_inner(255, None, 0, VAL_0356),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0611, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0357),
            CartNode::init_val_inner(255, None, 0, VAL_0358),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0614, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0359),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0616, VAL_0012),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0617, VAL_0071),
            CartNode::init_val_inner(255, None, 0, VAL_0360),
            CartNode::init_val_inner(255, None, 0, VAL_0361),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0620, VAL_0080),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0621, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0362),
            CartNode::init_val_inner(255, None, 0, VAL_0363),
            CartNode::init_val_inner(255, None, 0, VAL_0364),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0625, VAL_0012),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0626, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0365),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0628, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0366),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0630, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0367),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0632, VAL_0006),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0633, VAL_0075),
            CartNode::init_val_inner(255, None, 0, VAL_0368),
            CartNode::init_val_inner(255, None, 0, VAL_0369),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0636, VAL_0071),
            CartNode::init_val_inner(255, None, 0, VAL_0370),
            CartNode::init_val_inner(255, None, 0, VAL_0371),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0639, VAL_0009),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0640, VAL_0126),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0641, VAL_0038),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0642, VAL_0080),
            CartNode::init_val_inner(255, None, 0, VAL_0372),
            CartNode::init_val_inner(255, None, 0, VAL_0373),
            CartNode::init_val_inner(27, Some(CartOperation::Is), CTNODE_NO_0645, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0374),
            CartNode::init_val_inner(255, None, 0, VAL_0375),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0648, VAL_0080),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0649, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0376),
            CartNode::init_val_inner(255, None, 0, VAL_0377),
            CartNode::init_val_inner(255, None, 0, VAL_0378),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0653, VAL_0033),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0654, VAL_0080),
            CartNode::init_val_inner(255, None, 0, VAL_0379),
            CartNode::init_val_inner(255, None, 0, VAL_0380),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0657, VAL_0126),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0658, VAL_0018),
            CartNode::init_val_inner(255, None, 0, VAL_0381),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0660, VAL_0075),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0661, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0382),
            CartNode::init_val_inner(255, None, 0, VAL_0383),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0664, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0384),
            CartNode::init_val_inner(255, None, 0, VAL_0385),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0667, VAL_0006),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0668, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0386),
            CartNode::init_val_inner(255, None, 0, VAL_0387),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0671, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0388),
            CartNode::init_val_inner(255, None, 0, VAL_0389),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0674, VAL_0015),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0675, VAL_0006),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0676, VAL_0043),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0677, VAL_0006),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0678, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0390),
            CartNode::init_val_inner(255, None, 0, VAL_0391),
            CartNode::init_val_inner(255, None, 0, VAL_0392),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0682, VAL_0012),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0683, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0393),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0685, VAL_0187),
            CartNode::init_val_inner(255, None, 0, VAL_0394),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0687, VAL_0395),
            CartNode::init_val_inner(255, None, 0, VAL_0396),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0689, VAL_0085),
            CartNode::init_val_inner(255, None, 0, VAL_0397),
            CartNode::init_val_inner(255, None, 0, VAL_0398),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0692, VAL_0008),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0693, VAL_0399),
            CartNode::init_val_inner(255, None, 0, VAL_0400),
            CartNode::init_val_inner(255, None, 0, VAL_0401),
            CartNode::init_val_inner(13, Some(CartOperation::Is), CTNODE_NO_0696, VAL_0023),
            CartNode::init_val_inner(27, Some(CartOperation::Is), CTNODE_NO_0697, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0402),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0699, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0403),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0701, VAL_0404),
            CartNode::init_val_inner(255, None, 0, VAL_0405),
            CartNode::init_val_inner(255, None, 0, VAL_0406),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0704, VAL_0179),
            CartNode::init_val_inner(255, None, 0, VAL_0407),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0706, VAL_0215),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0707, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0408),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0709, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0409),
            CartNode::init_val_inner(10, Some(CartOperation::Less), CTNODE_NO_0711, VAL_0054),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0712, VAL_0009),
            CartNode::init_val_inner(15, Some(CartOperation::Less), CTNODE_NO_0713, VAL_0037),
            CartNode::init_val_inner(255, None, 0, VAL_0410),
            CartNode::init_val_inner(255, None, 0, VAL_0411),
            CartNode::init_val_inner(255, None, 0, VAL_0412),
            CartNode::init_val_inner(255, None, 0, VAL_0413),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0718, VAL_0075),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0719, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0414),
            CartNode::init_val_inner(255, None, 0, VAL_0415),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0722, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0416),
            CartNode::init_val_inner(255, None, 0, VAL_0417),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0725, VAL_0006),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0726, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0418),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0728, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0419),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0730, VAL_0006),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0731, VAL_0019),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0732, VAL_0420),
            CartNode::init_val_inner(25, Some(CartOperation::Is), CTNODE_NO_0733, VAL_0006),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0734, VAL_0018),
            CartNode::init_val_inner(255, None, 0, VAL_0421),
            CartNode::init_val_inner(255, None, 0, VAL_0422),
            CartNode::init_val_inner(255, None, 0, VAL_0423),
            CartNode::init_val_inner(255, None, 0, VAL_0424),
            CartNode::init_val_inner(255, None, 0, VAL_0425),
            CartNode::init_val_inner(255, None, 0, VAL_0426),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0741, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0427),
            CartNode::init_val_inner(255, None, 0, VAL_0428),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0744, VAL_0009),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0745, VAL_0012),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0746, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0429),
            CartNode::init_val_inner(255, None, 0, VAL_0430),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0749, VAL_0009),
            CartNode::init_val_inner(255, None, 0, VAL_0431),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0751, VAL_0006),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0752, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0432),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0754, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0433),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0756, VAL_0062),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0757, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0434),
            CartNode::init_val_inner(255, None, 0, VAL_0435),
            CartNode::init_val_inner(255, None, 0, VAL_0436),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0761, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0437),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0763, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0438),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0765, VAL_0126),
            CartNode::init_val_inner(255, None, 0, VAL_0439),
            CartNode::init_val_inner(255, None, 0, VAL_0440),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0768, VAL_0071),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0769, VAL_0028),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0770, VAL_0075),
            CartNode::init_val_inner(255, None, 0, VAL_0441),
            CartNode::init_val_inner(255, None, 0, VAL_0442),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0773, VAL_0054),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0774, VAL_0019),
            CartNode::init_val_inner(255, None, 0, VAL_0443),
            CartNode::init_val_inner(255, None, 0, VAL_0444),
            CartNode::init_val_inner(255, None, 0, VAL_0445),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0778, VAL_0111),
            CartNode::init_val_inner(13, Some(CartOperation::Is), CTNODE_NO_0779, VAL_0023),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0780, VAL_0033),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0781, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0446),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0783, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0447),
            CartNode::init_val_inner(255, None, 0, VAL_0448),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0786, VAL_0018),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0787, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0449),
            CartNode::init_val_inner(255, None, 0, VAL_0450),
            CartNode::init_val_inner(28, Some(CartOperation::Is), CTNODE_NO_0790, VAL_0193),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0791, VAL_0270),
            CartNode::init_val_inner(255, None, 0, VAL_0451),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0793, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0452),
            CartNode::init_val_inner(21, Some(CartOperation::Is), CTNODE_NO_0795, VAL_0075),
            CartNode::init_val_inner(24, Some(CartOperation::Less), CTNODE_NO_0796, VAL_0050),
            CartNode::init_val_inner(255, None, 0, VAL_0453),
            CartNode::init_val_inner(255, None, 0, VAL_0454),
            CartNode::init_val_inner(255, None, 0, VAL_0455),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0800, VAL_0033),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0801, VAL_0006),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0802, VAL_0054),
            CartNode::init_val_inner(255, None, 0, VAL_0456),
            CartNode::init_val_inner(26, Some(CartOperation::Is), CTNODE_NO_0804, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0457),
            CartNode::init_val_inner(255, None, 0, VAL_0458),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0807, VAL_0038),
            CartNode::init_val_inner(255, None, 0, VAL_0459),
            CartNode::init_val_inner(255, None, 0, VAL_0460),
            CartNode::init_val_inner(27, Some(CartOperation::Is), CTNODE_NO_0810, VAL_0013),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0811, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0461),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0813, VAL_0012),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0814, VAL_0006),
            CartNode::init_val_inner(16, Some(CartOperation::Less), CTNODE_NO_0815, VAL_0050),
            CartNode::init_val_inner(255, None, 0, VAL_0462),
            CartNode::init_val_inner(255, None, 0, VAL_0463),
            CartNode::init_val_inner(255, None, 0, VAL_0464),
            CartNode::init_val_inner(255, None, 0, VAL_0465),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0820, VAL_0046),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0821, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0466),
            CartNode::init_val_inner(255, None, 0, VAL_0467),
            CartNode::init_val_inner(7, Some(CartOperation::Is), CTNODE_NO_0824, VAL_0033),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0825, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0468),
            CartNode::init_val_inner(255, None, 0, VAL_0469),
            CartNode::init_val_inner(11, Some(CartOperation::Is), CTNODE_NO_0828, VAL_0006),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0829, VAL_0038),
            CartNode::init_val_inner(20, Some(CartOperation::Is), CTNODE_NO_0830, VAL_0006),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0831, VAL_0006),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0832, VAL_0270),
            CartNode::init_val_inner(255, None, 0, VAL_0470),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0834, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0471),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0836, VAL_0009),
            CartNode::init_val_inner(5, Some(CartOperation::Is), CTNODE_NO_0837, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0472),
            CartNode::init_val_inner(255, None, 0, VAL_0473),
            CartNode::init_val_inner(255, None, 0, VAL_0474),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0841, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0475),
            CartNode::init_val_inner(255, None, 0, VAL_0476),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0844, VAL_0395),
            CartNode::init_val_inner(255, None, 0, VAL_0477),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0846, VAL_0012),
            CartNode::init_val_inner(255, None, 0, VAL_0478),
            CartNode::init_val_inner(255, None, 0, VAL_0479),
            CartNode::init_val_inner(18, Some(CartOperation::Is), CTNODE_NO_0849, VAL_0013),
            CartNode::init_val_inner(255, None, 0, VAL_0480),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0851, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0481),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0853, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0482),
            CartNode::init_val_inner(6, Some(CartOperation::Less), CTNODE_NO_0855, VAL_0126),
            CartNode::init_val_inner(255, None, 0, VAL_0483),
            CartNode::init_val_inner(255, None, 0, VAL_0484),
            CartNode::init_val_inner(17, Some(CartOperation::Less), CTNODE_NO_0858, VAL_0054),
            CartNode::init_val_inner(9, Some(CartOperation::Is), CTNODE_NO_0859, VAL_0006),
            CartNode::init_val_inner(4, Some(CartOperation::Is), CTNODE_NO_0860, VAL_0008),
            CartNode::init_val_inner(255, None, 0, VAL_0485),
            CartNode::init_val_inner(22, Some(CartOperation::Less), CTNODE_NO_0862, VAL_0395),
            CartNode::init_val_inner(255, None, 0, VAL_0486),
            CartNode::init_val_inner(255, None, 0, VAL_0487),
            CartNode::init_val_inner(255, None, 0, VAL_0488),
            CartNode::init_val_inner(20, Some(CartOperation::Is), CTNODE_NO_0866, VAL_0006),
            CartNode::init_val_inner(255, None, 0, VAL_0489),
            CartNode::init_val_inner(255, None, 0, VAL_0490),
            CartNode::init_val_inner(8, Some(CartOperation::Is), CTNODE_NO_0869, VAL_0033),
            CartNode::init_val_inner(255, None, 0, VAL_0491),
            CartNode::init_val_inner(19, Some(CartOperation::Less), CTNODE_NO_0871, VAL_0038),
            CartNode::init_val_inner(255, None, 0, VAL_0492),
            CartNode::init_val_inner(255, None, 0, VAL_0493),
            CartNode::init_val_inner(255, None, 0, VAL_0494),
        ],
        [
            "Name",
            "emph_sil",
            "p.R:SylStructure.parent.parent.pbreak",
            "R:SylStructure.parent.accented",
            "n.ph_ctype",
            "p.ph_vlng",
            "R:SylStructure.parent.syl_codasize",
            "p.ph_ctype",
            "ph_ctype",
            "R:SylStructure.parent.syl_break",
            "R:SylStructure.parent.asyl_in",
            "ph_vlng",
            "p.p.ph_vfront",
            "seg_onsetcoda",
            "R:SylStructure.parent.parent.gpos",
            "R:SylStructure.parent.last_accent",
            "R:SylStructure.parent.sub_phrases",
            "pos_in_syl",
            "R:SylStructure.parent.R:Syllable.p.syl_break",
            "R:SylStructure.parent.R:Syllable.n.syl_onsetsize",
            "seg_onset_stop",
            "R:SylStructure.parent.position_type",
            "R:SylStructure.parent.syl_out",
            "p.p.ph_vlng",
            "R:SylStructure.parent.parent.word_numsyls",
            "seg_coda_fric",
            "n.n.ph_vheight",
            "n.n.ph_vfront",
            "ph_vheight",
        ],
    )
}

#[test]
fn test_durz_cart_tree() {
    let x = durz_cart_tree();
}
