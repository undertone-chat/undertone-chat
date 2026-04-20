use crossbeam_channel::Sender as CrossbeamSender;
use dioxus::prelude::Signal;
use dioxus::signals::WritableExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::audio::AudioCommand;
use crate::ui_events::UiEvent;

#[derive(Debug)]
pub enum ControlCommand {
    Connect(String),
    Join(u32),
    Disconnect,
}

#[derive(Debug)]
pub struct ControlConnection {
    cmd_rx: UnboundedReceiver<ControlCommand>,
    event_tx: UnboundedSender<UiEvent>,
    audio_tx: CrossbeamSender<AudioCommand>,
    stream: Option<TcpStream>,
}

impl ControlConnection {
    pub fn new(
        mut cmd_rx: UnboundedReceiver<ControlCommand>,
        event_tx: UnboundedSender<UiEvent>,
        audio_tx: CrossbeamSender<AudioCommand>,
    ) -> Self {
        Self {
            cmd_rx,
            event_tx,
            audio_tx,
            stream: None,
        }
    }

    pub async fn run(&mut self) {
        // let mut stream: Option<TcpStream> = None;

        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                ControlCommand::Connect(address) => {
                    if self.stream.is_some() {
                        tracing::warn!("Already connected!");
                        return;
                    }
                    let _ = self
                        .event_tx
                        .send(UiEvent::ConnectionStatus("Connecting...".into()));
                    match TcpStream::connect(&address).await {
                        Ok(tcp) => {
                            let _ = self
                                .event_tx
                                .send(UiEvent::ConnectionStatus("Connected!".into()));
                            self.stream = Some(tcp);
                            // Pass stream to handler in new thread.
                            tokio::spawn(async move {
                                stream_handler(tcp).await;
                            });
                            let _ = self.audio_tx.send(AudioCommand::StartStreaming(address));
                        }
                        Err(error) => {
                            let _ = self
                                .event_tx
                                .send(UiEvent::ConnectionStatus(format!("Error: {}", error)));
                        }
                    }
                }
                ControlCommand::Disconnect => {
                    self.stream = None;
                    let _ = self.event_tx.send(UiEvent::ConnectionLost);
                    let _ = self.audio_tx.send(AudioCommand::Shutdown);
                }
                any => {
                    tracing::debug!("Unhandled command: {:?}", any);
                }
            }
        }
    }
}

async fn stream_handler(mut socket: TcpStream) {
    // let Some(mut socket) = stream else {
    //     return;
    // };

    let mut buf = [0; 1024];
    tracing::debug!(
        "stream  handler started for connection to {}",
        socket.peer_addr().unwrap()
    );

    loop {
        socket.readable().await.unwrap();
        let n = match socket.read(&mut buf).await {
            Ok(0) => return,
            Ok(n) => n,
            Err(error) => {
                tracing::error!("Failed to read from socket; err= {:?}", error);
                return;
            }
        };

        let answer = std::str::from_utf8(&buf[0..n]).expect("some utf bs");
        tracing::debug!("Got: {}", answer);
    }
}

// pub async fn run_tcp_actor(
//     mut cmd_rx: UnboundedReceiver<ControlCommand>,
//     event_tx: UnboundedSender<UiEvent>, // UI Listener
//     audio_tx: CrossbeamSender<AudioCommand>,
// ) {
//     let mut stream: Option<TcpStream> = None;
//
//     // This loop stays alive independently of the UI components
//     while let Some(cmd) = cmd_rx.recv().await {
//         match cmd {
//             ControlCommand::Connect(address) => {
//                 tracing::debug!("Got Control Command Connect");
//                 let _ = event_tx.send(UiEvent::ConnectionStatus("Connecing...".into()));
//                 match TcpStream::connect(&address).await {
//                     Ok(tcp) => {
//                         let _ = event_tx.send(UiEvent::ConnectionStatus("Connected!".into()));
//                         stream = Some(tcp);
//                         let _ = audio_tx.send(AudioCommand::StartStreaming(address));
//                     }
//                     Err(e) => {
//                         let _ = event_tx.send(UiEvent::ConnectionStatus(format!("Error: {}", e)));
//                     }
//                 }
//             }
//             ControlCommand::Disconnect => {
//                 stream = None;
//                 let _ = event_tx.send(UiEvent::ConnectionLost);
//                 let _ = audio_tx.send(AudioCommand::Shutdown);
//             }
//             _ => {
//                 tracing::debug!("Unhandled Command Message!");
//             }
//         }
//     }
// }
