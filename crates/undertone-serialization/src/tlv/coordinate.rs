use super::*;
#[derive(Debug, Default)]
pub struct Coordinate {
    // Stores f32 coordinates in bit form for network transport.
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

impl Coordinate {}

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
        //buf.freeze();
        encode_tlv(Coordinate::TAG, Self::SIZE, buf.to_vec())
    }

    fn decode(buf: Bytes) -> Result<Self, TlvError> {
        let mut frame = decode_tlv(buf)?;
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
