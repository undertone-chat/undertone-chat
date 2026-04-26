use super::*;
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Pad2 {
    data: u16,
}

impl Tlv for Pad2 {
    const SIZE: u16 = 2;
    const TAG: Tag = Tag::Pad2;

    fn size(&self) -> u16 {
        Self::SIZE
    }

    fn encode(&self) -> Result<Bytes, TlvError> {
        let buf = self.data.to_be_bytes().to_vec();
        encode_tlv(Pad2::TAG, self.size(), buf)
    }

    fn decode(buf: Bytes) -> Result<Self, TlvError> {
        let mut frame = decode_tlv(buf)?;
        let value = match frame.data.try_get_u16() {
            Ok(v) => v,
            Err(error) => return Err(TlvError::TryGetError(error)),
        };

        Ok(Pad2 { data: value })
    }
}
