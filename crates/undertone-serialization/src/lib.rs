mod error;
mod reliable;
mod tlv;
mod util;

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u16 {
        const ACK = 0b0000_0001;
        const PRIORITY = 0b0000_0010;
    }
}
pub enum PacketType {}

/// Fixed size header (4 bytes)
pub struct PacketHeader {
    /// Package Size in Bytes
    size: u16,
    flags: u16,
}
