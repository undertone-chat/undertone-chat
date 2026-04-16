use rkyv::{Archive, Deserialize, Serialize, with::AsUnixTime};
use std::time::SystemTime;

#[derive(Archive, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Session {
    pub session_id: u32,
    pub user_name: String,
    #[rkyv(with = AsUnixTime)]
    pub connection_time: SystemTime,
}
