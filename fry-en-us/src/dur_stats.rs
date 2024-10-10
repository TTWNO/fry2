//! Durations lengths for various diphones.

/// A single duration stat for a given phoneme
#[derive(Copy, Clone, Debug)]
pub struct DurationStat<'a> {
    phone: &'a str,
    mean: f32,
    stddev: f32,
}
impl<'a> DurationStat<'a> {
    const fn init(phone: &'a str, mean: f32, stddev: f32) -> Self {
        Self {
            phone,
            mean,
            stddev,
        }
    }
}

static DUR_STAT_PAU: DurationStat<'static> = DurationStat::init("pau", 0.2, 0.1);
static DUR_STAT_ZH: DurationStat<'static> = DurationStat::init("zh", 0.152_593, 0.092_321);
static DUR_STAT_OY: DurationStat<'static> = DurationStat::init("oy", 0.160_374, 0.077_629);
static DUR_STAT_AW: DurationStat<'static> = DurationStat::init("aw", 0.159_485, 0.064_687);
static DUR_STAT_CH: DurationStat<'static> = DurationStat::init("ch", 0.135_828, 0.043_586);
static DUR_STAT_TH: DurationStat<'static> = DurationStat::init("th", 0.116_027, 0.054_892);
static DUR_STAT_UH: DurationStat<'static> = DurationStat::init("uh", 0.061_596, 0.023_654);
static DUR_STAT_G: DurationStat<'static> = DurationStat::init("g", 0.077_797, 0.027_193);
static DUR_STAT_AE: DurationStat<'static> = DurationStat::init("ae", 0.115_669, 0.047_921);
static DUR_STAT_SH: DurationStat<'static> = DurationStat::init("sh", 0.126_018, 0.023_275);
static DUR_STAT_V: DurationStat<'static> = DurationStat::init("v", 0.045_676, 0.017_954);
static DUR_STAT_EH: DurationStat<'static> = DurationStat::init("eh", 0.109_237, 0.046_925);
static DUR_STAT_W: DurationStat<'static> = DurationStat::init("w", 0.052_598, 0.024_618);
static DUR_STAT_EY: DurationStat<'static> = DurationStat::init("ey", 0.165_883, 0.075_7);
static DUR_STAT_DH: DurationStat<'static> = DurationStat::init("dh", 0.035_688, 0.021_493);
static DUR_STAT_NG: DurationStat<'static> = DurationStat::init("ng", 0.065_651, 0.022_119);
static DUR_STAT_UW: DurationStat<'static> = DurationStat::init("uw", 0.102_018, 0.047_394);
static DUR_STAT_ER: DurationStat<'static> = DurationStat::init("er", 0.100_174, 0.044_822);
static DUR_STAT_B: DurationStat<'static> = DurationStat::init("b", 0.063_457, 0.027_02);
static DUR_STAT_AH: DurationStat<'static> = DurationStat::init("ah", 0.062_256, 0.029_903);
static DUR_STAT_N: DurationStat<'static> = DurationStat::init("n", 0.058_944, 0.029_727);
static DUR_STAT_T: DurationStat<'static> = DurationStat::init("t", 0.074_067, 0.037_846);
static DUR_STAT_JH: DurationStat<'static> = DurationStat::init("jh", 0.083_748, 0.029_496);
static DUR_STAT_IH: DurationStat<'static> = DurationStat::init("ih", 0.062_962, 0.030_609);
static DUR_STAT_D: DurationStat<'static> = DurationStat::init("d", 0.050_917, 0.031_666);
static DUR_STAT_F: DurationStat<'static> = DurationStat::init("f", 0.096_548, 0.028_515);
static DUR_STAT_AO: DurationStat<'static> = DurationStat::init("ao", 0.091_841, 0.049_984);
static DUR_STAT_Y: DurationStat<'static> = DurationStat::init("y", 0.056_909, 0.027_74);
static DUR_STAT_K: DurationStat<'static> = DurationStat::init("k", 0.089_048, 0.040_764);
static DUR_STAT_Z: DurationStat<'static> = DurationStat::init("z", 0.088_234, 0.038_77);
static DUR_STAT_P: DurationStat<'static> = DurationStat::init("p", 0.099_085, 0.033_806);
static DUR_STAT_IY: DurationStat<'static> = DurationStat::init("iy", 0.126_115, 0.063_085);
static DUR_STAT_R: DurationStat<'static> = DurationStat::init("r", 0.052_082, 0.023_499);
static DUR_STAT_AA: DurationStat<'static> = DurationStat::init("aa", 0.109_23, 0.045_992);
static DUR_STAT_S: DurationStat<'static> = DurationStat::init("s", 0.108_565, 0.041_973);
static DUR_STAT_M: DurationStat<'static> = DurationStat::init("m", 0.074_447, 0.044_589);
static DUR_STAT_AY: DurationStat<'static> = DurationStat::init("ay", 0.151_095, 0.045_892);
static DUR_STAT_OW: DurationStat<'static> = DurationStat::init("ow", 0.146_084, 0.052_605);
static DUR_STAT_L: DurationStat<'static> = DurationStat::init("l", 0.065_292, 0.033_114);
static DUR_STAT_AX: DurationStat<'static> = DurationStat::init("ax", 0.053_852, 0.033_216);
static DUR_STAT_HH: DurationStat<'static> = DurationStat::init("hh", 0.067_775, 0.021_633);

static US_DUR_STATS: [DurationStat<'static>; 41] = [
    DUR_STAT_UH,
    DUR_STAT_HH,
    DUR_STAT_AO,
    DUR_STAT_V,
    DUR_STAT_IH,
    DUR_STAT_EY,
    DUR_STAT_JH,
    DUR_STAT_W,
    DUR_STAT_UW,
    DUR_STAT_AE,
    DUR_STAT_K,
    DUR_STAT_Y,
    DUR_STAT_L,
    DUR_STAT_NG,
    DUR_STAT_ZH,
    DUR_STAT_Z,
    DUR_STAT_M,
    DUR_STAT_IY,
    DUR_STAT_N,
    DUR_STAT_AH,
    DUR_STAT_ER,
    DUR_STAT_B,
    DUR_STAT_PAU,
    DUR_STAT_AW,
    DUR_STAT_P,
    DUR_STAT_CH,
    DUR_STAT_OW,
    DUR_STAT_DH,
    DUR_STAT_D,
    DUR_STAT_AX,
    DUR_STAT_R,
    DUR_STAT_EH,
    DUR_STAT_AY,
    DUR_STAT_OY,
    DUR_STAT_F,
    DUR_STAT_SH,
    DUR_STAT_S,
    DUR_STAT_G,
    DUR_STAT_TH,
    DUR_STAT_AA,
    DUR_STAT_T,
];
