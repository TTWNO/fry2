//! Check is symbol is a pronouncable word or not.
//! Uses Finite-State-Machines for checking forward or backwards for the right combinations characters.

use crate::aswd_state::State;
use fry_common::error::{FsmError, FsmInvalidIndex, FsmStateError};

/// From `lang/usenglish/us_aswd.c`
// fits into 9 bits
const FSM_ASWD_S_STATE: [usize; 74] = [
    0, 2, 23, 25, 29, 32, 38, 49, 56, 62, 69, 81, 87, 97, 115, 133, 140, 149, 167, 174, 185, 186,
    191, 198, 201, 204, 210, 214, 217, 222, 229, 242, 246, 248, 255, 259, 264, 267, 271, 276, 286,
    292, 295, 298, 300, 304, 310, 313, 317, 322, 329, 336, 344, 361, 368, 373, 380, 385, 394, 398,
    400, 408, 410, 413, 415, 418, 422, 424, 427, 438, 441, 444, 447, 449,
];

/// A finite state machine that represents the letters in a word.
#[derive(Clone, Debug)]
struct Fsm<const LEN: usize> {
    fsm: [Option<State>; LEN],
    idx: usize,
    next: usize,
}
impl<const LEN: usize> Fsm<LEN> {
    fn state(&self) -> Option<State> {
        self.fsm.get(self.idx).copied()?
    }
    /// Transision to a new state of the finite-state-machine.
    /// Returns the new index if it has changed, none otherwise.
    fn transition(&mut self, s: char) -> Option<usize> {
        // this will never panic because the indexes are verified upon creation
        let (diff, next) = self.fsm[self.next..]
            .iter()
            // get up until the next None state
            .map_while(|x| *x)
            .enumerate()
            // if the char of the state matches s, yield the offset i
            .find_map(|(i, x)| {
                if x.character == s {
                    Some((i, x.next_index))
                } else {
                    None
                }
            })?;
        self.idx = self.next + diff;
        self.next = next;
        Some(self.next)
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
                v if "aeiouy".contains(v) => 'V',
                s => s,
            };
            let diff = self.transition(symbol);
            match (diff, symbol) {
                (None, _) => return false,
                (Some(_), 'V') => return true,
                (Some(_), _) => continue,
            }
        }
        false
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
                v if "aeiouy".contains(v) => 'V',
                s => s,
            };
            let diff = self.transition(symbol);
            match (diff, symbol) {
                (None, _) => return false,
                (Some(_), 'V') => return true,
                (Some(_), _) => continue,
            }
        }
        false
    }
    /// Same as [`Self::const_init`], but panics of any of the error cases are reached.
    const fn const_init_unchecked(start: [Option<(usize, char)>; LEN]) -> Self {
        let res = Self::const_init(start);
        match res {
            Err(FsmError::FsmInvalidIndex(fii)) => const_panic::concat_panic!("The index selected: ", fii.ref_idx, " at index ", fii.idx, " is not contained in an array of size ", fii.len),
            Err(FsmError::FsmStateError(FsmStateError::NonAscii(ch))) => const_panic::concat_panic!("NonAscii: the state must contain a non-zero value for either the next index or ASCII char. Invalid char: ", ch),
            Ok(myself) => myself
        }
    }
    /// Add all the states for the FSM in one `const_initialization` step.
    /// This function also validates
    const fn const_init(start: [Option<(usize, char)>; LEN]) -> Result<Self, FsmError> {
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
            if item.0 > start.len() - 1 {
                return Err(FsmError::FsmInvalidIndex(FsmInvalidIndex {
                    idx: i,
                    ref_idx: item.0,
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
            idx: 0,
            next: 0,
        })
    }
}

/// From `lang/usenglish/us_aswd.c`
const FSM_ASWD_S_TRANS: Fsm<454> = Fsm::const_init_unchecked([
    Some((FSM_ASWD_S_STATE[1], '#')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'j')),
    Some((FSM_ASWD_S_STATE[3], 'q')),
    Some((FSM_ASWD_S_STATE[4], 'v')),
    Some((FSM_ASWD_S_STATE[5], 'b')),
    Some((FSM_ASWD_S_STATE[6], 'z')),
    Some((FSM_ASWD_S_STATE[7], 'f')),
    Some((FSM_ASWD_S_STATE[8], 'x')),
    Some((FSM_ASWD_S_STATE[9], 'p')),
    Some((FSM_ASWD_S_STATE[10], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[11], 'c')),
    Some((FSM_ASWD_S_STATE[12], 'k')),
    Some((FSM_ASWD_S_STATE[13], 't')),
    Some((FSM_ASWD_S_STATE[14], 'l')),
    Some((FSM_ASWD_S_STATE[15], 'g')),
    Some((FSM_ASWD_S_STATE[16], 'd')),
    Some((FSM_ASWD_S_STATE[17], 's')),
    Some((FSM_ASWD_S_STATE[18], 'r')),
    Some((FSM_ASWD_S_STATE[19], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[4], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'b')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'z')),
    Some((FSM_ASWD_S_STATE[2], 'f')),
    Some((FSM_ASWD_S_STATE[21], 'c')),
    Some((FSM_ASWD_S_STATE[22], 't')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[23], 'd')),
    Some((FSM_ASWD_S_STATE[24], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[25], 'f')),
    Some((FSM_ASWD_S_STATE[26], 'p')),
    Some((FSM_ASWD_S_STATE[27], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'x')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[24], 'p')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'b')),
    Some((FSM_ASWD_S_STATE[28], 'p')),
    Some((FSM_ASWD_S_STATE[29], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'k')),
    Some((FSM_ASWD_S_STATE[30], 't')),
    Some((FSM_ASWD_S_STATE[28], 'g')),
    Some((FSM_ASWD_S_STATE[24], 'd')),
    Some((FSM_ASWD_S_STATE[31], 's')),
    Some((FSM_ASWD_S_STATE[32], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'z')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[28], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'k')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[33], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'b')),
    Some((FSM_ASWD_S_STATE[34], 'z')),
    Some((FSM_ASWD_S_STATE[35], 'f')),
    Some((FSM_ASWD_S_STATE[2], 'x')),
    Some((FSM_ASWD_S_STATE[28], 'p')),
    Some((FSM_ASWD_S_STATE[36], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[24], 'c')),
    Some((FSM_ASWD_S_STATE[4], 'k')),
    Some((FSM_ASWD_S_STATE[26], 't')),
    Some((FSM_ASWD_S_STATE[37], 'l')),
    Some((FSM_ASWD_S_STATE[24], 'g')),
    Some((FSM_ASWD_S_STATE[38], 'd')),
    Some((FSM_ASWD_S_STATE[39], 's')),
    Some((FSM_ASWD_S_STATE[27], 'r')),
    Some((FSM_ASWD_S_STATE[40], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'j')),
    Some((FSM_ASWD_S_STATE[2], 'v')),
    Some((FSM_ASWD_S_STATE[2], 'b')),
    Some((FSM_ASWD_S_STATE[41], 'z')),
    Some((FSM_ASWD_S_STATE[24], 'p')),
    Some((FSM_ASWD_S_STATE[42], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[43], 'c')),
    Some((FSM_ASWD_S_STATE[44], 'k')),
    Some((FSM_ASWD_S_STATE[45], 't')),
    Some((FSM_ASWD_S_STATE[4], 'l')),
    Some((FSM_ASWD_S_STATE[24], 'g')),
    Some((FSM_ASWD_S_STATE[24], 'd')),
    Some((FSM_ASWD_S_STATE[46], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[24], 'g')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[27], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'z')),
    Some((FSM_ASWD_S_STATE[2], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[47], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'd')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[4], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'v')),
    Some((FSM_ASWD_S_STATE[5], 'b')),
    Some((FSM_ASWD_S_STATE[2], 'z')),
    Some((FSM_ASWD_S_STATE[48], 'f')),
    Some((FSM_ASWD_S_STATE[49], 'p')),
    Some((FSM_ASWD_S_STATE[50], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[11], 'c')),
    Some((FSM_ASWD_S_STATE[51], 'k')),
    Some((FSM_ASWD_S_STATE[52], 't')),
    Some((FSM_ASWD_S_STATE[53], 'l')),
    Some((FSM_ASWD_S_STATE[54], 'g')),
    Some((FSM_ASWD_S_STATE[55], 'd')),
    Some((FSM_ASWD_S_STATE[4], 's')),
    Some((FSM_ASWD_S_STATE[56], 'r')),
    Some((FSM_ASWD_S_STATE[57], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'v')),
    Some((FSM_ASWD_S_STATE[2], 'h')),
    Some((FSM_ASWD_S_STATE[2], 't')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'j')),
    Some((FSM_ASWD_S_STATE[2], 'z')),
    Some((FSM_ASWD_S_STATE[58], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'g')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[4], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    None,
    Some((FSM_ASWD_S_STATE[59], 'z')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 't')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[4], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'z')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[43], 'p')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'h')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[4], 't')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[60], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[61], 'v')),
    Some((FSM_ASWD_S_STATE[62], 'f')),
    Some((FSM_ASWD_S_STATE[2], 'x')),
    Some((FSM_ASWD_S_STATE[2], 'p')),
    Some((FSM_ASWD_S_STATE[63], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[43], 'g')),
    Some((FSM_ASWD_S_STATE[2], 'd')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[4], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'r')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'v')),
    Some((FSM_ASWD_S_STATE[2], 't')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[32], 't')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'f')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[64], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'g')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[4], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[4], 'b')),
    Some((FSM_ASWD_S_STATE[4], 'p')),
    Some((FSM_ASWD_S_STATE[2], 'k')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[24], 'g')),
    Some((FSM_ASWD_S_STATE[2], 'd')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[65], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[66], 't')),
    Some((FSM_ASWD_S_STATE[62], 'd')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 't')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    None,
    Some((FSM_ASWD_S_STATE[67], 'c')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'N')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    None,
    Some((FSM_ASWD_S_STATE[2], 't')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'f')),
    Some((FSM_ASWD_S_STATE[27], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'p')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[28], 'p')),
    Some((FSM_ASWD_S_STATE[4], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'k')),
    Some((FSM_ASWD_S_STATE[68], 't')),
    Some((FSM_ASWD_S_STATE[2], 'g')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[26], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'b')),
    Some((FSM_ASWD_S_STATE[2], 'z')),
    Some((FSM_ASWD_S_STATE[2], 'f')),
    Some((FSM_ASWD_S_STATE[2], 'x')),
    Some((FSM_ASWD_S_STATE[28], 'p')),
    Some((FSM_ASWD_S_STATE[69], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[24], 'c')),
    Some((FSM_ASWD_S_STATE[2], 't')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'g')),
    Some((FSM_ASWD_S_STATE[70], 'd')),
    Some((FSM_ASWD_S_STATE[4], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'v')),
    Some((FSM_ASWD_S_STATE[2], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'g')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[65], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'd')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[71], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'h')),
    Some((FSM_ASWD_S_STATE[66], 's')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[58], 'h')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'g')),
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[27], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 't')),
    Some((FSM_ASWD_S_STATE[2], 'g')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[32], 's')),
    None,
    Some((FSM_ASWD_S_STATE[72], 'z')),
    Some((FSM_ASWD_S_STATE[2], 'k')),
    Some((FSM_ASWD_S_STATE[73], 't')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'l')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'g')),
    None,
    Some((FSM_ASWD_S_STATE[32], 's')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 's')),
    None,
    Some((FSM_ASWD_S_STATE[2], 's')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'f')),
    Some((FSM_ASWD_S_STATE[2], 'x')),
    Some((FSM_ASWD_S_STATE[2], 'p')),
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[2], 'l')),
    Some((FSM_ASWD_S_STATE[43], 'g')),
    Some((FSM_ASWD_S_STATE[24], 'd')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'c')),
    Some((FSM_ASWD_S_STATE[2], 'g')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    None,
    Some((FSM_ASWD_S_STATE[2], 'w')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
    Some((FSM_ASWD_S_STATE[43], 't')),
    None,
    Some((FSM_ASWD_S_STATE[2], 't')),
    Some((FSM_ASWD_S_STATE[2], 'r')),
    Some((FSM_ASWD_S_STATE[2], 'N')),
    Some((FSM_ASWD_S_STATE[20], 'V')),
    None,
]);

const FSM_ASWD_P_STATE: [usize; 35] = [
    0, 2, 23, 25, 34, 38, 46, 55, 59, 64, 72, 81, 85, 102, 108, 111, 120, 126, 133, 137, 138, 140,
    142, 145, 149, 152, 155, 158, 161, 166, 171, 176, 193, 195, 197,
];

const FSM_ASWD_P_TRANS: Fsm<203> = Fsm::const_init_unchecked([
    Some((FSM_ASWD_P_STATE[1], '#')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[2], 'q')),
    Some((FSM_ASWD_P_STATE[3], 'z')),
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[4], 'v')),
    Some((FSM_ASWD_P_STATE[5], 'k')),
    Some((FSM_ASWD_P_STATE[6], 't')),
    Some((FSM_ASWD_P_STATE[7], 'w')),
    Some((FSM_ASWD_P_STATE[8], 'f')),
    Some((FSM_ASWD_P_STATE[9], 'g')),
    Some((FSM_ASWD_P_STATE[10], 'p')),
    Some((FSM_ASWD_P_STATE[11], 'l')),
    Some((FSM_ASWD_P_STATE[12], 's')),
    Some((FSM_ASWD_P_STATE[13], 'h')),
    Some((FSM_ASWD_P_STATE[14], 'r')),
    Some((FSM_ASWD_P_STATE[15], 'd')),
    Some((FSM_ASWD_P_STATE[16], 'b')),
    Some((FSM_ASWD_P_STATE[17], 'c')),
    Some((FSM_ASWD_P_STATE[18], 'N')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'v')),
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[20], 's')),
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[21], 'd')),
    Some((FSM_ASWD_P_STATE[2], 'b')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[2], 'N')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[2], 'v')),
    Some((FSM_ASWD_P_STATE[2], 'k')),
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[22], 's')),
    Some((FSM_ASWD_P_STATE[23], 'h')),
    Some((FSM_ASWD_P_STATE[24], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[24], 'r')),
    Some((FSM_ASWD_P_STATE[2], 'N')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 't')),
    Some((FSM_ASWD_P_STATE[25], 'f')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 's')),
    Some((FSM_ASWD_P_STATE[4], 'h')),
    Some((FSM_ASWD_P_STATE[24], 'r')),
    Some((FSM_ASWD_P_STATE[2], 'N')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'q')),
    Some((FSM_ASWD_P_STATE[26], 'z')),
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[2], 'v')),
    Some((FSM_ASWD_P_STATE[4], 'k')),
    Some((FSM_ASWD_P_STATE[27], 't')),
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'f')),
    Some((FSM_ASWD_P_STATE[21], 'g')),
    Some((FSM_ASWD_P_STATE[28], 'p')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[29], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[30], 'c')),
    Some((FSM_ASWD_P_STATE[2], 'N')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 's')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'z')),
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[2], 'v')),
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'j')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'z')),
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[4], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[31], 'c')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    None,
    Some((FSM_ASWD_P_STATE[32], 'c')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'r')),
    None,
    Some((FSM_ASWD_P_STATE[32], 'c')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'z')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[33], 'c')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[32], 'c')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[34], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'q')),
    Some((FSM_ASWD_P_STATE[2], 'v')),
    Some((FSM_ASWD_P_STATE[2], 'k')),
    Some((FSM_ASWD_P_STATE[2], 't')),
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'f')),
    Some((FSM_ASWD_P_STATE[4], 'g')),
    Some((FSM_ASWD_P_STATE[14], 'p')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'h')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[2], 'd')),
    Some((FSM_ASWD_P_STATE[27], 'b')),
    Some((FSM_ASWD_P_STATE[4], 'c')),
    Some((FSM_ASWD_P_STATE[2], 'N')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'h')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'z')),
    None,
    Some((FSM_ASWD_P_STATE[2], 'w')),
    Some((FSM_ASWD_P_STATE[2], 'l')),
    Some((FSM_ASWD_P_STATE[2], 'r')),
    Some((FSM_ASWD_P_STATE[2], 'N')),
    Some((FSM_ASWD_P_STATE[19], 'V')),
    None,
]);

/// These tests can be confirmed by opening a file in and enabling logging in Flite.
/// This can be found in the file in flite at: `/lang/usenglish/us_aswd.rs` in function
/// `fsm_transition`.
/// There is a line for debugging. If that line is uncomented, you will be able to verify the
/// output of the functions.
/// To determine if the word is being checked forward and backward, it is suggested you add
/// debugging to the top of the `is_word_pre` and `is_word_suf` functions, like is done in this
/// library.
///
/// The output is interpreted to create many of these tests.
#[cfg(test)]
mod tests {
    use super::*;
    /// These tests are based on output from running Flite with a debugging line and then converted
    /// to these tests.
    #[test]
    fn test_world_fsm_suffix() {
        let mut fsm = FSM_ASWD_S_TRANS.clone();
        // must explicitly start the FSM
        // This should not need to be done
        assert_ne!(fsm.transition('#'), None);
        assert_eq!(fsm.state().unwrap().character, '#');
        assert_eq!(fsm.state().unwrap().next_index, 2);
        // start from the back of the string;
        // last letter of world = d
        assert_ne!(fsm.transition('d'), None);
        assert_eq!(fsm.state().unwrap().character, 'd');
        assert_eq!(fsm.state().unwrap().next_index, 140);
        // next back is l
        assert_ne!(fsm.transition('l'), None);
        assert_eq!(fsm.state().unwrap().character, 'l');
        assert_eq!(fsm.state().unwrap().next_index, 313);
        // and next is r
        assert_ne!(fsm.transition('r'), None);
        assert_eq!(fsm.state().unwrap().character, 'r');
        assert_eq!(fsm.state().unwrap().next_index, 23);
        // and next is o
        // V = Vowel
        assert_ne!(fsm.transition('V'), None);
        assert_eq!(fsm.state().unwrap().character, 'V');
        assert_eq!(fsm.state().unwrap().next_index, 185);
        // this should not find anything
        assert_eq!(fsm.transition('w'), None);
    }
    #[test]
    fn test_world_fsm_prefix() {
        let mut fsm = FSM_ASWD_P_TRANS.clone();
        // must explicitly start the FSM
        // This should not need to be done
        assert_ne!(fsm.transition('#'), None);
        assert_eq!(fsm.state().unwrap().character, '#');
        assert_eq!(fsm.state().unwrap().next_index, 2);
        // first letter = w
        assert_ne!(fsm.transition('w'), None);
        assert_eq!(fsm.state().unwrap().character, 'w');
        assert_eq!(fsm.state().unwrap().next_index, 55);
        // second letter = o
        // V = vowel
        assert_ne!(fsm.transition('V'), None);
        assert_eq!(fsm.state().unwrap().character, 'V');
        assert_eq!(fsm.state().unwrap().next_index, 137);
        // third letter = r
        // this should lead to a None; r is not found
        assert_eq!(fsm.transition('r'), None);
    }
}
