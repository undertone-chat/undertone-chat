use super::*;
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Pad1 {
    data: u8,
}

impl Tlv for Pad1 {
    const SIZE: u16 = 1;
    const TAG: Tag = Tag::Pad2;

    fn size(&self) -> u16 {
        Self::SIZE
    }

    fn encode(&self) -> Result<Bytes, TlvError> {
        // Encode our value.
        let value_buf = self.data.to_le_bytes().to_vec();
        encode_tlv(Pad1::TAG, Self::SIZE, value_buf)
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
