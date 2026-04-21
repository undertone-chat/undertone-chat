use std::time::SystemTime;

use rkyv::{Archive, Deserialize, Serialize, rancor::Error, util::AlignedVec, with::AsUnixTime};

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct ControlPacket {
    header: ControlHeader,
    body: ControlBody,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct ControlHeader {
    version: u16,
    request_id: u64,
    require_ack: bool,
    #[rkyv(with = AsUnixTime)]
    time: SystemTime,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[repr(u16)]
pub enum ControlBody {
    Ack(Ack) = 1,
    KeepAlive = 2,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Ack {
    ack_request_id: u64,
    is_ok: bool,
}

impl ControlPacket {
    pub fn new(header: ControlHeader, body: ControlBody) -> Self {
        Self { header, body }
    }

    pub fn header(&self) -> &ControlHeader {
        &self.header
    }

    pub fn body(&self) -> &ControlBody {
        &self.body
    }
}

impl ControlHeader {
    pub fn new(version: u16, request_id: u64, require_ack: bool, time: SystemTime) -> Self {
        Self {
            version,
            request_id,
            require_ack,
            time,
        }
    }
}

/// Encode a ControlPacket into aligned bytes.
pub fn encode_packet(packet: &ControlPacket) -> AlignedVec {
    // encode the packet!
    rkyv::to_bytes::<Error>(packet).unwrap()
}

/// Decode a ControlPacket safely with checks using aligned bytes.
pub fn decode_packet(bytes: &AlignedVec) -> ControlPacket {
    rkyv::from_bytes::<ControlPacket, Error>(bytes).unwrap()
}
