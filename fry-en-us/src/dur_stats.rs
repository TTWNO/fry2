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
static DUR_STAT_ZH: DurationStat<'static> = DurationStat::init("zh", 0.152593, 0.092321);
static DUR_STAT_OY: DurationStat<'static> = DurationStat::init("oy", 0.160374, 0.077629);
static DUR_STAT_AW: DurationStat<'static> = DurationStat::init("aw", 0.159485, 0.064687);
static DUR_STAT_CH: DurationStat<'static> = DurationStat::init("ch", 0.135828, 0.043586);
static DUR_STAT_TH: DurationStat<'static> = DurationStat::init("th", 0.116027, 0.054892);
static DUR_STAT_UH: DurationStat<'static> = DurationStat::init("uh", 0.061596, 0.023654);
static DUR_STAT_G: DurationStat<'static> = DurationStat::init("g", 0.077797, 0.027193);
static DUR_STAT_AE: DurationStat<'static> = DurationStat::init("ae", 0.115669, 0.047921);
static DUR_STAT_SH: DurationStat<'static> = DurationStat::init("sh", 0.126018, 0.023275);
static DUR_STAT_V: DurationStat<'static> = DurationStat::init("v", 0.045676, 0.017954);
static DUR_STAT_EH: DurationStat<'static> = DurationStat::init("eh", 0.109237, 0.046925);
static DUR_STAT_W: DurationStat<'static> = DurationStat::init("w", 0.052598, 0.024618);
static DUR_STAT_EY: DurationStat<'static> = DurationStat::init("ey", 0.165883, 0.0757);
static DUR_STAT_DH: DurationStat<'static> = DurationStat::init("dh", 0.035688, 0.021493);
static DUR_STAT_NG: DurationStat<'static> = DurationStat::init("ng", 0.065651, 0.022119);
static DUR_STAT_UW: DurationStat<'static> = DurationStat::init("uw", 0.102018, 0.047394);
static DUR_STAT_ER: DurationStat<'static> = DurationStat::init("er", 0.100174, 0.044822);
static DUR_STAT_B: DurationStat<'static> = DurationStat::init("b", 0.063457, 0.02702);
static DUR_STAT_AH: DurationStat<'static> = DurationStat::init("ah", 0.062256, 0.029903);
static DUR_STAT_N: DurationStat<'static> = DurationStat::init("n", 0.058944, 0.029727);
static DUR_STAT_T: DurationStat<'static> = DurationStat::init("t", 0.074067, 0.037846);
static DUR_STAT_JH: DurationStat<'static> = DurationStat::init("jh", 0.083748, 0.029496);
static DUR_STAT_IH: DurationStat<'static> = DurationStat::init("ih", 0.062962, 0.030609);
static DUR_STAT_D: DurationStat<'static> = DurationStat::init("d", 0.050917, 0.031666);
static DUR_STAT_F: DurationStat<'static> = DurationStat::init("f", 0.096548, 0.028515);
static DUR_STAT_AO: DurationStat<'static> = DurationStat::init("ao", 0.091841, 0.049984);
static DUR_STAT_Y: DurationStat<'static> = DurationStat::init("y", 0.056909, 0.02774);
static DUR_STAT_K: DurationStat<'static> = DurationStat::init("k", 0.089048, 0.040764);
static DUR_STAT_Z: DurationStat<'static> = DurationStat::init("z", 0.088234, 0.03877);
static DUR_STAT_P: DurationStat<'static> = DurationStat::init("p", 0.099085, 0.033806);
static DUR_STAT_IY: DurationStat<'static> = DurationStat::init("iy", 0.126115, 0.063085);
static DUR_STAT_R: DurationStat<'static> = DurationStat::init("r", 0.052082, 0.023499);
static DUR_STAT_AA: DurationStat<'static> = DurationStat::init("aa", 0.10923, 0.045992);
static DUR_STAT_S: DurationStat<'static> = DurationStat::init("s", 0.108565, 0.041973);
static DUR_STAT_M: DurationStat<'static> = DurationStat::init("m", 0.074447, 0.044589);
static DUR_STAT_AY: DurationStat<'static> = DurationStat::init("ay", 0.151095, 0.045892);
static DUR_STAT_OW: DurationStat<'static> = DurationStat::init("ow", 0.146084, 0.052605);
static DUR_STAT_L: DurationStat<'static> = DurationStat::init("l", 0.065292, 0.033114);
static DUR_STAT_AX: DurationStat<'static> = DurationStat::init("ax", 0.053852, 0.033216);
static DUR_STAT_HH: DurationStat<'static> = DurationStat::init("hh", 0.067775, 0.021633);

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
