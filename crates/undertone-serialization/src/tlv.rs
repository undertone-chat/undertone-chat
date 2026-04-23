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
}

impl TryFrom<u16> for Tag {
    type Error = TlvError;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Tag::Empty as u16 => Ok(Tag::Empty),
            x if x == Tag::Pad1 as u16 => Ok(Tag::Pad1),
            x if x == Tag::Pad2 as u16 => Ok(Tag::Pad2),
            _ => Err(TlvError::InvalidTag(v)),
        }
    }
}
/// Expected Tlv shape is
/// ```
///  tag  size data
/// [u16][u16][Bytes]
/// ```
/// where data length (bytes) = size
trait Tlv {
    const SIZE: u16;
    fn tag(&self) -> Tag;
    fn size(&self) -> u16;
    fn encode(&self) -> Result<Bytes, TlvError>;
    fn decode(buf: Bytes) -> Result<Self, TlvError>
    where
        Self: std::marker::Sized;
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Pad1 {
    data: u8,
}

impl Tlv for Pad1 {
    const SIZE: u16 = 1;

    fn tag(&self) -> Tag {
        Tag::Pad1
    }

    fn size(&self) -> u16 {
        Self::SIZE
    }

    fn encode(&self) -> Result<Bytes, TlvError> {
        // Encode our value.
        let value_buf = self.data.to_le_bytes().to_vec();
        encode_tlv(Tag::Pad1, Self::SIZE, value_buf)
    }

    fn decode(buf: Bytes) -> Result<Self, TlvError> {
        let mut frame = decode_tlv(buf)?;
        let value = match frame.data.try_get_u8() {
            Ok(v) => v,
            Err(error) => return Err(TlvError::TryGetError(error)),
        };

        Ok(Pad1 { data: value })
    }
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
            expected: 8u16,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tlv_round_trip() {
        let pad1 = Pad1 { data: 69 };
        let buf_result = pad1.encode();
        assert!(buf_result.is_ok());
        let buf = buf_result.unwrap();

        let result = Pad1::decode(buf).unwrap();
        assert_eq!(pad1, result);
    }
}
