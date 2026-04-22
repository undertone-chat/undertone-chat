use uuid::Timestamp;
pub use uuid::Uuid;

/// Type Aliased UUID v7 to store time and provide unique identifier, providing an
/// abstraction layer to allow easier changes in future protocols.
#[derive(rkyv::Archive, rkyv::Serialize)]
pub(crate) struct MessageId(Uuid);

impl MessageId {
    pub(super) fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn value(&self) -> Uuid {
        self.0
    }

    /// Get the Uuid Timestamp for when this message was sent give or take some nanoseconds.
    pub fn time(&self) -> Option<Timestamp> {
        self.get_timestamp()
    }
}

impl std::ops::Deref for MessageId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn message_id_should_return_valid_time() {
    let message_id = MessageId::new();
    assert!(message_id.time().is_some())
}
