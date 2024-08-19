use derive_more::{Error, Display, From};

#[derive(Clone, Copy, Debug, Display, Error, From)]
pub enum FsmError {
    #[display("FsmMaxLengthExceeded: the FSM may not exceed 512")]
    FsmMaxLengthExceeded,
    FsmInvalidIndex(FsmInvalidIndex),
    FsmStateError(FsmStateError),
}

#[derive(Clone, Copy, Debug, Display, Error)]
pub enum FsmStateError {
    #[display("NonZero: the state must contain a non-zero value for either the next index or ASCII char")]
    NonZero,
    #[display("NonAscii: the state must contain a valid ASCII char; invalid char: {_0}")]
    #[error(ignore)]
    NonAscii(char),
}

#[derive(Clone, Copy, Debug, Display, Error, From)]
pub enum FryError {
    Fsm(FsmError),
}

#[derive(Clone, Copy, Debug, Display, Error)]
#[display("The index: {ref_idx:} refered to at idx: {idx:} was longer than the length: {len:}")]
pub struct FsmInvalidIndex {
    pub idx: usize, 
    pub ref_idx: usize,
    pub len: usize, 
}
