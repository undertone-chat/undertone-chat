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

impl Tag {
    fn get_tlv_size(&self) -> u16 {
        match self {
            Tag::Empty => 0,
            Tag::Pad1 => pad1::Pad1::SIZE,
            Tag::Pad2 => pad2::Pad2::SIZE,
            Tag::Coordinate => coordinate::Coordinate::SIZE,
        }
    }
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

/// Attempt to extract a Tlv from the buffer if we are expecting some
///
/// Returns None if there are not enough bytes to extract a valid Tlv,
/// does not mean there isn't one but we will let the calling code make that
/// call.
pub fn try_parse_frame_streaming(buf: &mut Bytes) -> Result<Option<TlvFrame>, TlvError> {
    // Make sure there is something left in the buffer for us to evaluate, should at least be 8 bytes
    // for tag and size. We may be waiting on stream to buffer as well so well just say nothin yet.
    if buf.len() < TLV_TAG_SIZE + TLV_SIZE_SIZE {
        // Nope what ever is left is not for us.
        println!("Returning none since buf.len isnt bigger than our min size!");
        return Ok(None);
    }

    let mut cur = buf.clone();

    let raw_tag = cur.try_get_u16().map_err(|_| TlvError::IncompleteHeader)?;
    let tag = Tag::try_from(raw_tag)?;

    let size = cur.try_get_u16().map_err(|_| TlvError::IncompleteHeader)?;

    if tag.get_tlv_size() != size {
        return Err(TlvError::SizeMismatch {
            expected: tag.get_tlv_size(),
            got: size as usize,
        });
    }
    // If so we can try to pull out the data if there are enough bytes.

    if cur.remaining() < size as usize {
        println!("Not enough remaining buffer to get the data from.");
        return Ok(None);
    }

    let data = cur.split_to(size as usize);

    // Replace the previous handle with our new advanced handle so we don't lose our place.
    *buf = cur;

    Ok(Some(TlvFrame { tag, size, data }))
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
    fn can_extract_frames_from_stream() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&encode_tlv(Tag::Pad1, Pad1::SIZE, [0u8; 1].to_vec()).unwrap());
        raw.extend_from_slice(&encode_tlv(Tag::Pad2, Pad2::SIZE, [0u8; 2].to_vec()).unwrap());
        let coordinate = Coordinate::new(0.1, 0.2, 0.3);
        raw.extend_from_slice(
            &encode_tlv(
                Tag::Coordinate,
                Coordinate::SIZE,
                coordinate.to_be_bytes().to_vec(),
            )
            .unwrap(),
        );

        let mut read_buf = Bytes::from(raw);
        let first_frame = try_parse_frame_streaming(&mut read_buf).unwrap().unwrap();
        assert_eq!(first_frame.tag, Tag::Pad1);
        let second_frame = try_parse_frame_streaming(&mut read_buf).unwrap().unwrap();
        assert_eq!(second_frame.tag, Tag::Pad2);
        let mut third_frame = try_parse_frame_streaming(&mut read_buf).unwrap().unwrap();
        assert_eq!(third_frame.tag, Tag::Coordinate);
        assert_eq!(third_frame.size, Coordinate::SIZE);
        assert_eq!(
            coordinate,
            Coordinate::from_tlv_frame(&mut third_frame).unwrap()
        );

        // Ensure we actually consumed everything and nothing is dangling!
        assert!(!read_buf.has_remaining());
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
