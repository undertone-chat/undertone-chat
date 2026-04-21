use rkyv::{Archive, Deserialize, Serialize, rancor::Error, util::AlignedVec};
use std::{fmt, time};

/// Control packet for transmission over TCP control channels.
#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct ControlPacket {
    /// Packet header for useful common information.
    header: ControlHeader,

    /// Body contains the packet type with associated data.
    body: ControlBody,
}

/// Fixed size header which contains useful data about the associated packet.
#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct ControlHeader {
    /// Undertone protocol version.
    version: u16,

    /// Monotic request identifier.
    request_id: u64,

    /// Does the sender expect an acknowledgment from the receiver.
    require_ack: bool,

    /// Monotic time since server start.
    timestamp: time::Duration,
}

/// Fancy formatting for debugging!
impl fmt::Display for ControlHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "request_id: {}, required_ack: {}, time_sent: {:?}",
            self.request_id, self.require_ack, self.timestamp
        )
    }
}

/// Same thing but for the Archived version used by rkyv
impl fmt::Display for ArchivedControlHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "request_id: {}, required_ack: {}, time_sent: {:?}",
            self.request_id, self.require_ack, self.timestamp
        )
    }
}

#[derive(Archive, Serialize, Deserialize, Debug)]
#[repr(u16)]
pub enum ControlBody {
    Ack(Ack) = 1,
    KeepAlive = 2,
}

/// Response Acknowledgement body for commands requiring ack.
#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Ack {
    /// The original request_id from the `ControlHeader`.
    ack_request_id: u64,

    /// Was the operation or instruction received OK.
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
    pub fn new(
        version: u16,
        request_id: u64,
        require_ack: bool,
        timestamp: time::Duration,
    ) -> Self {
        Self {
            version,
            request_id,
            require_ack,
            timestamp,
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
