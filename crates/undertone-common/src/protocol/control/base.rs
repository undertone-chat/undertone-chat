use std::time::SystemTime;

use rkyv::{Archive, Deserialize, Serialize, with::AsUnixTime};

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
