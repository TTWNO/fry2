use core::num::NonZeroU16;
use fry_common::error::FsmStateError;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct State {
    pub next_index: usize,
    pub character: char,
}
impl State {
    pub const fn const_init_unchecked(state_ref: usize, ascii_chr: char) -> Self {
        match Self::const_init(state_ref, ascii_chr) {
            Err(FsmStateError::NonAscii(ch)) => const_panic::concat_panic!("NonAscii: the state must contain a valid ASCII char. Invalid char: ", ch),
            Ok(state) => state,
        }
    }
    // TODO: use more const_panic::concat_assert to keep more local errors at compile time
    // TODO: make all const fns with non-const variations (with logging)
    pub const fn const_init(next_index: usize, character: char) -> Result<Self, FsmStateError> {
        if !character.is_ascii() {
            return Err(FsmStateError::NonAscii(character));
        }
        Ok(Self {
            next_index,
            character
        })
    }
}
