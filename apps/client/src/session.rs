use tokio::net::TcpStream;
use uuid::Uuid;

use crate::Message;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Uuid,
    nickname: String,
}

impl User {
    pub fn new(id: Uuid, nickname: String) -> Self {
        Self { id, nickname }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ServerAddress {
    pub addr: String,
    pub port: u16,
}

impl Default for ServerAddress {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1".to_string(),
            port: 9990,
        }
    }
}

#[derive(Debug)]
pub struct Session {
    pub server_addr: ServerAddress,
    pub connection: Option<TcpStream>,
    pub user: User,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            server_addr: ServerAddress::default(),
            connection: None,
            user: User {
                id: Uuid::new_v4(),
                nickname: "Test".to_string(),
            },
        }
    }
}

impl Session {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Connect => self.connect(),
            Message::Disconnect => self.disconnect(),
        }
    }

    fn connect(&self) {
        println!("connect");
    }

    fn disconnect(&self) {
        println!("disconnect");
    }
}
