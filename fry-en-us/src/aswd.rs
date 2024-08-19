//! Check is symbol is a pronouncable word or not.
//! Uses Finite-State-Machines for checking forward or backwards for the right combinations characters.

use core::num::{
    NonZeroU16,
    NonZeroUsize,
};
use crate::error::{
    FsmError,
    FsmStateError,
    FsmInvalidIndex,
};

/// From `lang/usenglish/us_aswd.c`
// fits into 9 bits
const FSM_ASWD_S_STATE: [u16; 74] = [
    0, 2, 23, 25, 29, 32, 38, 49, 56, 62, 69, 81, 87, 97, 115, 133, 140, 149, 167, 174, 185, 186, 191, 198, 201, 204, 210, 214, 217, 222, 229, 242, 246, 248, 255, 259, 264, 267, 271, 276, 286, 292, 295, 298, 300, 304, 310, 313, 317, 322, 329, 336, 344, 361, 368, 373, 380, 385, 394, 398, 400, 408, 410, 413, 415, 418, 422, 424, 427, 438, 441, 444, 447, 449,
];

/// A fconst_inite state machine that represents the letters in a word.
struct Fsm<const LEN: usize>{
    fsm: [Option<State>; LEN],
    idx: usize,
}
impl<const LEN: usize> Fsm<LEN> {
    /// Transision to a new state of the finite-state-machine.
    /// Returns the relative index change, if the state is moved. 
    fn transition(&mut self, s: char) -> Option<NonZeroUsize> {
        let idx_off = self.fsm[self.idx..]
            .into_iter()
            // get up until the next None state
            .map_while(|x| *x)
            .enumerate()
            // if the char of the state matches s, yield the offset i
            .find_map(|(i,x)| if x.chr() == s {
                Some(i)
            } else { None })
            // if not found, the offset is 0
            .unwrap_or_default();
        self.idx += idx_off;
        NonZeroUsize::new(idx_off)
    }
    /// Check if `word` has a word-like prefix
    fn is_word_pre(&mut self, word: &str) -> bool {
        // used to move the state machine out of its default state
        // this should not have to happen
        // TODO: fix by moving all values in the const declaration down two, and remeving the rest.
        self.transition('#');
        for chr in word.chars() {
            let symbol = match chr {
                'n' | 'm' => 'N',
                _v if "aeiouy".contains(_v) => 'V',
                s => s,
            };
            let diff = self.transition(symbol);
            match (diff, symbol) {
                (None, _) => return false,
                (Some(_), 'V') => return true,
                (Some(_), _) => continue,
            }
        }
        return false;
    }
    /// Check if `word` has a word-like suffix
    fn is_word_suf(&mut self, word: &str) -> bool {
        // used to move the state machine out of its default state
        // this should not have to happen
        // TODO: fix by moving all values in the const declaration down two, and remeving the rest.
        self.transition('#');
        for chr in word.chars().rev() {
            let symbol = match chr {
                'n' | 'm' => 'N',
                _v if "aeiouy".contains(_v) => 'V',
                s => s,
            };
            let diff = self.transition(symbol);
            match (diff, symbol) {
                (None, _) => return false,
                (Some(_), 'V') => return true,
                (Some(_), _) => continue,
            }
        }
        return false;
    }
    /// Same as [`Self::const_init`], but panics of any of the error cases are reached.
    const fn const_init_unchecked(start: [Option<(u16, char)>; LEN]) -> Self {
        let res = Self::const_init(start);
        match res {
            Err(FsmError::FsmMaxLengthExceeded) => panic!("The maximum length of the FSM must be <= 511"),
            Err(FsmError::FsmInvalidIndex(fii)) => const_panic::concat_panic!("The index selected: ", fii.ref_idx, " at index ", fii.idx, " is not contained in an array of size ", fii.len),
            Err(FsmError::FsmStateError(FsmStateError::NonAscii(ch))) => const_panic::concat_panic!("NonAscii: the state must contain a non-zero value for either the next index or ASCII char. Invalid char: ", ch),
            Err(FsmError::FsmStateError(FsmStateError::NonZero)) => panic!("NonZero: the state must contain a valid ASCII char or an index which is not 0"),
            Ok(myself) => myself
        }
    }
    /// Add all the states for the FSM in one const_initialization step.
    /// This function also validates 
    const fn const_init(start: [Option<(u16, char)>; LEN]) -> Result<Self, FsmError> {
        if LEN > 0x1FF /* 512 - 1 */ {
            return Err(FsmError::FsmMaxLengthExceeded);
        }
        let mut i = 0;
        // const_initialize array
        let mut state: [Option<State>; LEN] = [None; LEN];
        // cannot use iterator or for loop in const
        while i < LEN {
            let Some(item) = start[i] else {
                state[i] = None;
                i += 1;
                continue;
            };
            // if reference is out of bounds
            if item.0 as usize > start.len()-1 {
                return Err(FsmError::FsmInvalidIndex(FsmInvalidIndex {
                    idx: i,
                    ref_idx: item.0 as usize,
                    len: start.len(),
                }));
            }
            // once validated, assign the items
            // shortened once .ok_or() gets constified:
            // https://github.com/rust-lang/rust/issues/91930
            state[i] = match State::const_init(item.0, item.1) {
                Ok(state) => Some(state),
                Err(e) => return Err(FsmError::FsmStateError(e)),
            };
            i += 1;
        }
        Ok(Self {
            fsm: state,
            idx: 0
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct State(NonZeroU16);
impl State {
    const fn const_init_unchecked(state_ref: u16, ascii_chr: char) -> Self {
        match Self::const_init(state_ref, ascii_chr) {
            Err(FsmStateError::NonZero) => panic!("NonZero: the state must contain a non-zero value for either the next index or ASCII char"),
            Err(FsmStateError::NonAscii(ch)) => const_panic::concat_panic!("NonAscii: the state must contain a valid ASCII char. Invalid char: ", ch),
            Ok(state) => state,
        }
    }
    // TODO: use more const_panic::concat_assert to keep more local errors at compile time
    // TODO: make all const fns with non-const variations (with logging)
    const fn const_init(state_ref: u16, ascii_chr: char) -> Result<Self, FsmStateError> {
        if !ascii_chr.is_ascii() {
            return Err(FsmStateError::NonAscii(ascii_chr));
        }
        // move the index ref to the last 9 bits
        // then, take the last 7 bits of ascii_chr (it must be a valid ascii char)
        let num = ((state_ref << 7) as u16) | (ascii_chr as u16 & 0x7F);
        let non0_num = NonZeroU16::new(num);
        match non0_num {
            None => Err(FsmStateError::NonZero),
            Some(non0) => Ok(State(non0)),
        }
    }
    /// Get the char contained within the data.
    #[inline(always)] 
    fn chr(&self) -> char {
        // mask for the last 7 bits, cast to u8, then convert into a char
        char::from((self.0.get() & 0x7F) as u8)
    }
    /// Get the index pointed to by the state.
    #[inline(always)] 
    fn idx(&self) -> usize {
        // get the first 9 bits of the inner value, then cast to the native pointer size
        (self.0.get() >> 7) as usize
    }
}

/// From `lang/usenglish/us_aswd.c`
const FSM_ASWD_S_TRANS3: Fsm<3> = Fsm::const_init_unchecked([
    Some((2, '#')),
    None,
    Some((0, 'j')),
]);

/// From `lang/usenglish/us_aswd.c`
const FSM_ASWD_S_TRANS2: [u16; 454] = [
    // the version from flite does * 128 instead of << 7
    // and + some_number, istead of & some_number;
    // this is simpler since the number added is always under 127
    // basically, FSM_ASWD_STATE[x] takes the first 9 bits, then the rest is an additonal 7 bits
    (FSM_ASWD_S_STATE[1] << 7) & '#' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_S_STATE[3] << 7) & 'q' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'v' as u16,
    (FSM_ASWD_S_STATE[5] << 7) & 'b' as u16,
    (FSM_ASWD_S_STATE[6] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[7] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[8] << 7) & 'x' as u16,
    (FSM_ASWD_S_STATE[9] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[10] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[11] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[12] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[13] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[14] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[15] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[16] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[17] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[18] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[19] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[4] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'b' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[21] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[22] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[23] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[25] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[26] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[27] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'x' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[24] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'b' as u16,
    (FSM_ASWD_S_STATE[28] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[29] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[30] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[28] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[31] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[32] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[28] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[33] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'b' as u16,
    (FSM_ASWD_S_STATE[34] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[35] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'x' as u16,
    (FSM_ASWD_S_STATE[28] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[36] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[26] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[37] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[38] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[39] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[27] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[40] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'b' as u16,
    (FSM_ASWD_S_STATE[41] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[42] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[43] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[44] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[45] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[46] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[27] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[47] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_S_STATE[5] << 7) & 'b' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[48] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[49] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[50] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[11] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[51] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[52] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[53] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[54] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[55] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[56] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[57] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[58] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    0,
    (FSM_ASWD_S_STATE[59] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[43] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[4] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[60] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[61] << 7) & 'v' as u16,
    (FSM_ASWD_S_STATE[62] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'x' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[63] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[43] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[32] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[64] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[4] << 7) & 'b' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[65] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[66] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[62] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    0,
    (FSM_ASWD_S_STATE[67] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[27] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[28] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[68] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[26] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'b' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'x' as u16,
    (FSM_ASWD_S_STATE[28] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[69] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[70] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[4] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[65] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[71] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[66] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[58] << 7) & 'h' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[27] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[32] << 7) & 's' as u16,
    0,
    (FSM_ASWD_S_STATE[72] << 7) & 'z' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'k' as u16,
    (FSM_ASWD_S_STATE[73] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    0,
    (FSM_ASWD_S_STATE[32] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'f' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'x' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'p' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_S_STATE[43] << 7) & 'g' as u16,
    (FSM_ASWD_S_STATE[24] << 7) & 'd' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'c' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'g' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_S_STATE[43] << 7) & 't' as u16,
    0,
    (FSM_ASWD_S_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_S_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_S_STATE[20] << 7) & 'V' as u16,
    0
];

const FSM_ASWD_P_STATE: [u16; 35] = [
    0, 2, 23, 25, 34, 38, 46, 55, 59, 64, 72, 81, 85, 102, 108, 111, 120, 126, 133, 137, 138, 140, 142, 145, 149, 152, 155, 158, 161, 166, 171, 176, 193, 195, 197
];

const FSM_AWSD_P_TRANS: [u16; 203] = [
    (FSM_ASWD_P_STATE[1] << 7) & '#' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'x' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'q' as u16,
    (FSM_ASWD_P_STATE[3] << 7) & 'z' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_P_STATE[4] << 7) & 'v' as u16,
    (FSM_ASWD_P_STATE[5] << 7) & 'k' as u16,
    (FSM_ASWD_P_STATE[6] << 7) & 't' as u16,
    (FSM_ASWD_P_STATE[7] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[8] << 7) & 'f' as u16,
    (FSM_ASWD_P_STATE[9] << 7) & 'g' as u16,
    (FSM_ASWD_P_STATE[10] << 7) & 'p' as u16,
    (FSM_ASWD_P_STATE[11] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[12] << 7) & 's' as u16,
    (FSM_ASWD_P_STATE[13] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[14] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[15] << 7) & 'd' as u16,
    (FSM_ASWD_P_STATE[16] << 7) & 'b' as u16,
    (FSM_ASWD_P_STATE[17] << 7) & 'c' as u16,
    (FSM_ASWD_P_STATE[18] << 7) & 'N' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[20] << 7) & 's' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[21] << 7) & 'd' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'b' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'k' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[22] << 7) & 's' as u16,
    (FSM_ASWD_P_STATE[23] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[24] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[24] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_P_STATE[25] << 7) & 'f' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_P_STATE[4] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[24] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'q' as u16,
    (FSM_ASWD_P_STATE[26] << 7) & 'z' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_P_STATE[4] << 7) & 'k' as u16,
    (FSM_ASWD_P_STATE[27] << 7) & 't' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'f' as u16,
    (FSM_ASWD_P_STATE[21] << 7) & 'g' as u16,
    (FSM_ASWD_P_STATE[28] << 7) & 'p' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[29] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[30] << 7) & 'c' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 's' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'j' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[4] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[31] << 7) & 'c' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    0,
    (FSM_ASWD_P_STATE[32] << 7) & 'c' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    0,
    (FSM_ASWD_P_STATE[32] << 7) & 'c' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'z' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[33] << 7) & 'c' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[32] << 7) & 'c' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[34] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'q' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'v' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'k' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 't' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'f' as u16,
    (FSM_ASWD_P_STATE[4] << 7) & 'g' as u16,
    (FSM_ASWD_P_STATE[14] << 7) & 'p' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'd' as u16,
    (FSM_ASWD_P_STATE[27] << 7) & 'b' as u16,
    (FSM_ASWD_P_STATE[4] << 7) & 'c' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'h' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'z' as u16,
    0,
    (FSM_ASWD_P_STATE[2] << 7) & 'w' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'l' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'r' as u16,
    (FSM_ASWD_P_STATE[2] << 7) & 'N' as u16,
    (FSM_ASWD_P_STATE[19] << 7) & 'V' as u16,
    0,
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_option_nonzero_optimization() {
        assert_eq!(size_of::<Option<State>>(), size_of::<u16>());
    }
}

