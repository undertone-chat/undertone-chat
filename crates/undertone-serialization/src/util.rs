#![cfg_attr(debug_assertions, allow(dead_code))]

use std::time;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TimeStamp {
    seconds: u64,
    nanoseconds: u32,
}

impl TimeStamp {
    pub fn new(seconds: u64, nanoseconds: u32) -> Self {
        Self {
            seconds,
            nanoseconds,
        }
    }

    pub fn from_duration(duration: &time::Duration) -> Self {
        Self {
            seconds: duration.as_secs(),
            nanoseconds: duration.subsec_nanos(),
        }
    }

    pub fn to_duration(self) -> time::Duration {
        time::Duration::new(self.seconds, self.nanoseconds)
    }

    pub fn to_bytes(self) -> [u8; 12] {
        let mut buf = [0u8; 12];
        let b1 = self.seconds.to_be_bytes();
        let b2 = self.nanoseconds.to_be_bytes();
        buf[..8].copy_from_slice(&b1);
        buf[8..].copy_from_slice(&b2);
        buf
    }
}
