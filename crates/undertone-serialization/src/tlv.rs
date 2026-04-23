mod coordinate;
mod pad1;
mod pad2;

use crate::error::TlvError;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::result::Result;

const TLV_TAG_SIZE: usize = 2;
const TLV_SIZE_SIZE: usize = 2;
const TLV_MAX_SIZE: usize = 255;

struct TlvFrame {
    tag: Tag,
    size: u16,
    data: Bytes,
}

/// Tlv Tag to indicate the type of the following Tlv
#[derive(Debug, Default, PartialEq)]
#[repr(u16)]
#[non_exhaustive]
enum Tag {
    #[default]
    Empty = 0,
    Pad1 = 1, // 1 byte pad for alignment
    Pad2 = 2,
    Coordinate = 3,
}

impl TryFrom<u16> for Tag {
    type Error = TlvError;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Tag::Empty as u16 => Ok(Tag::Empty),
            x if x == Tag::Pad1 as u16 => Ok(Tag::Pad1),
            x if x == Tag::Pad2 as u16 => Ok(Tag::Pad2),
            x if x == Tag::Coordinate as u16 => Ok(Tag::Coordinate),
            _ => Err(TlvError::InvalidTag(v)),
        }
    }
}

trait Tlv {
    const SIZE: u16;
    const TAG: Tag;
    fn size(&self) -> u16;
    fn encode(&self) -> Result<Bytes, TlvError>;
    fn decode(buf: Bytes) -> Result<Self, TlvError>
    where
        Self: std::marker::Sized;
}

fn encode_tlv(tag: Tag, size: u16, data: Vec<u8>) -> Result<Bytes, TlvError> {
    if data.len() != size.into() {
        return Err(TlvError::SizeMismatch {
            expected: size,
            got: data.len(),
        });
    }

    let mut buf = BytesMut::with_capacity(TLV_TAG_SIZE + TLV_SIZE_SIZE + (size as usize));
    buf.put_u16(tag as u16);
    buf.put_u16(size);
    buf.put_slice(&data[..]);

    if buf.len() >= TLV_MAX_SIZE {
        return Err(TlvError::ExceedsMaxSize);
    }

    Ok(buf.freeze())
}

fn decode_tlv(mut buf: Bytes) -> Result<TlvFrame, TlvError> {
    if buf.len() < TLV_TAG_SIZE + TLV_SIZE_SIZE {
        return Err(TlvError::MinimumLength {
            expected: (TLV_TAG_SIZE + TLV_SIZE_SIZE) as u16,
            got: buf.len(),
        });
    }
    let tag_u16 = match buf.split_to(TLV_TAG_SIZE).try_get_u16() {
        Ok(value) => Tag::try_from(value),
        Err(error) => {
            return Err(TlvError::TryGetError(error));
        }
    };

    // Confirm the tag is valid.
    let tag = tag_u16?;

    let size = match buf.split_to(TLV_SIZE_SIZE).try_get_u16() {
        Ok(value) => value,
        Err(error) => {
            return Err(TlvError::TryGetError(error));
        }
    };

    if buf.len() != size.into() {
        return Err(TlvError::SizeMismatch {
            expected: size,
            got: buf.len(),
        });
    }

    Ok(TlvFrame {
        tag,
        size,
        data: buf,
    })
}

#[allow(unused)]
macro_rules! tlv_tests {
    ($($name:ident: $type:ty,)*) => {
        $(
            mod $name {
                use super::*;

                #[test]
                fn round_trip() {
                    let tlv = <$type>::default();
                    let buf = tlv.encode().unwrap();
                    let decoded = <$type>::decode(buf).unwrap();
                    assert_eq!(tlv,decoded);
                }
            }
            )*
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tlv::{coordinate::Coordinate, pad1::Pad1, pad2::Pad2};

    tlv_tests! {
        pad1: Pad1,
        pad2: Pad2,
        coordinate: Coordinate,
    }

    #[test]
    fn encode_fails_on_wrong_size() {
        assert!(encode_tlv(Tag::Pad1, 69, [0u8; 1].to_vec()).is_err());
    }

    #[test]
    fn decode_fails_on_too_small() {
        assert!(decode_tlv(Bytes::from([0u8; 1].to_vec())).is_err());
    }
}
