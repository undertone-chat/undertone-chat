use super::*;
#[derive(Default, Debug, PartialEq, Eq)]
struct Pad2 {
    data: u16,
}

impl Tlv for Pad2 {
    const SIZE: u16 = 2;

    fn tag(&self) -> Tag {
        Tag::Pad2
    }

    fn size(&self) -> u16 {
        Self::SIZE
    }

    fn encode(&self) -> Result<Bytes, TlvError> {
        let buf = self.data.to_be_bytes().to_vec();
        encode_tlv(self.tag(), self.size(), buf)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip() {
        let start = Pad2 { data: 42601 };
        let buf = start.encode().unwrap();
        let finish = Pad2::decode(buf).unwrap();
        assert_eq!(start, finish);
    }
}
