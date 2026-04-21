use rkyv::{Archive, Serialize, util::AlignedVec};

#[repr(u8)]
pub enum AudioPacketFlags {
    SubMix = 0b0000_0001,
    Positional = 0b0000_0010,
    Effects = 0b0000_0100,
}

#[derive(Archive, Serialize)]
pub struct AudioFrame {
    pub sequence: u64,
    pub flags: u8,
    pub size: u16,
    pub payload: bytes,
}

#[derive(Archive, Serialize)]
pub struct SubmixData {
    pub mix_id: u16,
}
