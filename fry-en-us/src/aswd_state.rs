use crate::error::{FryError as Error, FsmStateError};
use core::num::NonZeroU16;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct State(NonZeroU16);
impl State {
    pub const fn const_init_unchecked(state_ref: u16, ascii_chr: char) -> Self {
        match Self::const_init(state_ref, ascii_chr) {
            Err(FsmStateError::NonZero) => panic!("NonZero: the state must contain a non-zero value for either the next index or ASCII char"),
            Err(FsmStateError::NonAscii(ch)) => const_panic::concat_panic!("NonAscii: the state must contain a valid ASCII char. Invalid char: ", ch),
            Ok(state) => state,
        }
    }
    // TODO: use more const_panic::concat_assert to keep more local errors at compile time
    // TODO: make all const fns with non-const variations (with logging)
    pub const fn const_init(state_ref: u16, ascii_chr: char) -> Result<Self, FsmStateError> {
        if !ascii_chr.is_ascii() {
            return Err(FsmStateError::NonAscii(ascii_chr));
        }
        // move the index ref to the last 9 bits
        // then, take the last 7 bits of ascii_chr (it must be a valid ascii char)
        let num = (state_ref << 7) | (ascii_chr as u16 & 0x7F);
        let non0_num = NonZeroU16::new(num);
        match non0_num {
            None => Err(FsmStateError::NonZero),
            Some(non0) => Ok(State(non0)),
        }
    }
    /// Get the char contained within the data.
    #[inline]
    pub fn chr(&self) -> char {
        // mask for the last 7 bits, cast to u8, then convert into a char
        char::from((self.0.get() & 0x7F) as u8)
    }
    /// Get the index pointed to by the state.
    #[inline]
    pub fn idx(&self) -> usize {
        // get the first 9 bits of the inner value, then cast to the native pointer size
        (self.0.get() >> 7) as usize
    }
}

/// This implementation of [`Debug`] shows the value of the [`Self::chr`] and [`Self::idx`]
/// functions instead of the inner value.
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        f.debug_struct("State")
            .field("char", &self.chr())
            .field("index", &self.idx())
            .finish()
    }
}
