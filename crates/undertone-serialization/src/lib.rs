#![expect(dead_code)]

mod error;
mod message_id;
use error::MessageError;
use message_id::MessageId;

pub use rkyv::util::AlignedVec;
const UDP_MAX_SIZE: u16 = 1400; // Max bytes for safe datagram

#[derive(Default, rkyv::Serialize, rkyv::Archive)]
#[repr(u8)]
pub enum MessagePayload {
    #[default]
    None = 0,
    Control = 1,
    Ack = 2,
}

/// Frame for messages, duh.
#[derive(rkyv::Archive, rkyv::Serialize)]
struct Frame {
    /// Unique message identifier.
    message_id: MessageId,
    /// Payload of the message wrapped in the appropriate types.
    payload: MessagePayload,
    /// Does this message require an Ack from the receiver
    ack_required: bool,
}

impl Frame {
    pub fn builder() -> FrameBuilder {
        FrameBuilder::default()
    }

    pub fn ack_required(&self) -> bool {
        self.ack_required
    }
}

#[derive(Default)]
struct FrameBuilder {
    ack_required: bool,
    payload: MessagePayload,
}

impl FrameBuilder {
    pub fn new(payload: MessagePayload) -> FrameBuilder {
        FrameBuilder {
            payload,
            ack_required: false,
        }
    }

    pub fn ack_required(mut self, value: bool) -> FrameBuilder {
        self.ack_required = value;
        self
    }

    pub fn build(self) -> Result<Frame, MessageError> {
        Ok(Frame {
            message_id: MessageId::new(),
            payload: self.payload,
            ack_required: self.ack_required,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn builder_returns_desired_frame() {
        let build_result = FrameBuilder::new(MessagePayload::Ack)
            .ack_required(true)
            .build();
        assert!(build_result.is_ok());
        let frame = build_result.unwrap();
        assert!(frame.ack_required);
        assert!(frame.message_id.time().is_some());
    }
}
