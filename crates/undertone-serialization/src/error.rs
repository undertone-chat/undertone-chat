use bytes::TryGetError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TlvError {
    #[error("Tlv size mismatch, expected=({expected:?}) got=({got:?})")]
    SizeMismatch { expected: u16, got: usize },

    #[error("Tlv exceeds max size (255bytes)")]
    ExceedsMaxSize,

    #[error("Tlv error on decode: {0}")]
    TryGetError(TryGetError),

    #[error("Tlv Invalid Tag: {0}")]
    InvalidTag(u16),

    #[error("Tlv minimum length not met. expected={expected:?} got={got:?}")]
    MinimumLength { expected: u16, got: usize },
}
