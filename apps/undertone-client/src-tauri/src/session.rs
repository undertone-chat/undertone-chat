pub mod logs;
use std::sync::Mutex;

pub type SessionState = Mutex<Session>;

#[derive(Default, Debug)]
pub struct Session {
    session_id: u32,
}

impl Session {
    pub fn new(session_id: u32) -> Self {
        Self { session_id }
    }

    pub fn id(&self) -> u32 {
        self.session_id
    }
}
