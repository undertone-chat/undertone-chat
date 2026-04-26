// use core::slice::SlicePattern;

use super::*;
#[derive(Debug, Default)]
pub struct Coordinate {
    x: f32,
    y: f32,
    z: f32,
}
impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f32::EPSILON
            && (self.y - other.y).abs() < f32::EPSILON
            && (self.z - other.z).abs() < f32::EPSILON
    }
}

impl Eq for Coordinate {}

impl Coordinate {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_be_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(12);
        buf.put_f32(self.x);
        buf.put_f32(self.y);
        buf.put_f32(self.z);

        buf.freeze()
    }

    pub fn from_tlv_frame(frame: &mut TlvFrame) -> Result<Coordinate, TlvError> {
        let x = match frame.data.try_get_f32() {
            Ok(v) => v,
            Err(error) => return Err(TlvError::TryGetError(error)),
        };

        let y = match frame.data.try_get_f32() {
            Ok(v) => v,
            Err(error) => return Err(TlvError::TryGetError(error)),
        };

        let z = match frame.data.try_get_f32() {
            Ok(v) => v,
            Err(error) => return Err(TlvError::TryGetError(error)),
        };

        Ok(Coordinate { x, y, z })
    }
}

impl Tlv for Coordinate {
    const SIZE: u16 = 12;
    const TAG: Tag = Tag::Coordinate;

    fn size(&self) -> u16 {
        Self::SIZE
    }

    fn encode(&self) -> Result<Bytes, TlvError> {
        // Encode our value.
        let mut buf = BytesMut::new();
        buf.put_f32(self.x);
        buf.put_f32(self.y);
        buf.put_f32(self.z);

        encode_tlv(Coordinate::TAG, Self::SIZE, buf.to_vec())
    }

    fn decode(buf: Bytes) -> Result<Self, TlvError> {
        let mut frame = decode_tlv(buf)?;

        Self::from_tlv_frame(&mut frame)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_eq() {
        let a = Coordinate {
            x: 0.1,
            y: 0.2,
            z: 0.3,
        };
        let b = Coordinate {
            x: 0.1,
            y: 0.1 + 0.1,
            z: 0.0 + 0.3,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn should_return_neq() {
        let a = Coordinate {
            x: 0.1,
            y: 0.1,
            z: 0.1,
        };
        let b = Coordinate {
            x: 0.1,
            y: 0.1,
            z: 0.1001,
        };
        assert_ne!(a, b);
    }
}
