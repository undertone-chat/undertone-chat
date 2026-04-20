use crossbeam_channel::Sender as CrossbeamSender;
use dioxus::prelude::Signal;
use dioxus::signals::WritableExt;
use tokio::io::AsyncWriteExt;
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
    audio_tx: UnboundedSender<AudioCommand>,
}

impl ControlConnection {
    pub fn new(
        mut cmd_rx: UnboundedReceiver<ControlCommand>,
        event_tx: UnboundedSender<UiEvent>,
        audio_tx: UnboundedSender<AudioCommand>,
    ) -> Self {
        Self {
            cmd_rx,
            event_tx,
            audio_tx,
        }
    }

    pub async fn run(&mut self) {
        let mut stream: Option<TcpStream> = None;

        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                ControlCommand::Connect(address) => {
                    let _ = self
                        .event_tx
                        .send(UiEvent::ConnectionStatus("Connecting...".into()));
                    match TcpStream::connect(&address).await {
                        Ok(tcp) => {
                            let _ = self
                                .event_tx
                                .send(UiEvent::ConnectionStatus("Connected!".into()));
                            stream = Some(tcp);
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
                    stream = None;
                    let _ = self.event_tx.send(UiEvent::ConnectionLost);
                    let _ = self.audio_tx.send(AudioCommand::Shutdown);
                }
                _ => {
                    tracing::debug!("Unhandled command.");
                }
            }
        }
    }
}

pub async fn run_tcp_actor(
    mut cmd_rx: UnboundedReceiver<ControlCommand>,
    event_tx: UnboundedSender<UiEvent>, // UI Listener
    audio_tx: CrossbeamSender<AudioCommand>,
) {
    let mut stream: Option<TcpStream> = None;

    // This loop stays alive independently of the UI components
    while let Some(cmd) = cmd_rx.recv().await {
        match cmd {
            ControlCommand::Connect(address) => {
                tracing::debug!("Got Control Command Connect");
                let _ = event_tx.send(UiEvent::ConnectionStatus("Connecing...".into()));
                match TcpStream::connect(&address).await {
                    Ok(tcp) => {
                        let _ = event_tx.send(UiEvent::ConnectionStatus("Connected!".into()));
                        stream = Some(tcp);
                        let _ = audio_tx.send(AudioCommand::StartStreaming(address));
                    }
                    Err(e) => {
                        let _ = event_tx.send(UiEvent::ConnectionStatus(format!("Error: {}", e)));
                    }
                }
            }
            ControlCommand::Disconnect => {
                stream = None;
                let _ = event_tx.send(UiEvent::ConnectionLost);
                let _ = audio_tx.send(AudioCommand::Shutdown);
            }
            _ => {
                tracing::debug!("Unhandled Command Message!");
            }
        }
    }
}
