use crossbeam_channel::Sender as CrossbeamSender;
use dioxus::hooks::UnboundedReceiver;
use dioxus::prelude::Signal;
use dioxus::signals::WritableExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use undertone_common::AlignedVec;
use undertone_common::protocol::control::{ControlPacket, decode_packet, encode_packet};

use crate::audio::AudioCommand;
use crate::ui_events::UiEvent;

#[derive(Debug)]
pub enum ControlCommand {
    Connect(String),
    Disconnect,
    Send(ControlPacket),
}

enum SessionCommand {
    Send(ControlPacket),
    Shutdown,
}

struct SessionHandle {
    tx: tokio::sync::mpsc::Sender<SessionCommand>,
    task: tokio::task::JoinHandle<()>,
}

pub struct ControlConnection {
    cmd_rx: UnboundedReceiver<ControlCommand>,
    event_tx: UnboundedSender<UiEvent>,
    audio_tx: crossbeam_channel::Sender<AudioCommand>,
    session: Option<SessionHandle>,
    next_request_id: u64,
}

impl ControlConnection {
    pub fn new(
        cmd_rx: UnboundedReceiver<ControlCommand>,
        event_tx: UnboundedSender<UiEvent>,
        audio_tx: crossbeam_channel::Sender<AudioCommand>,
    ) -> Self {
        Self {
            cmd_rx,
            event_tx,
            audio_tx,
            session: None,
            next_request_id: 1,
        }
    }

    pub async fn run(mut self) {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                ControlCommand::Connect(addr) => self.connect(addr).await,
                ControlCommand::Disconnect => self.disconnect().await,
                ControlCommand::Send(packet) => self.send(packet).await,
            }
        }

        self.disconnect().await;
    }

    async fn connect(&mut self, addr: String) {
        if self.session.is_some() {
            let _ = self
                .event_tx
                .send(UiEvent::ConnectionStatus("Already Connected".into()));
            return;
        }

        let _ = self
            .event_tx
            .send(UiEvent::ConnectionStatus("Connecting...".into()));

        match TcpStream::connect(&addr).await {
            Ok(stream) => {
                let (tx, rx) = mpsc::channel(64);

                let event_tx = self.event_tx.clone();
                let audio_tx = self.audio_tx.clone();

                let task = tokio::spawn(async move {
                    run_session(stream, rx, event_tx, audio_tx).await;
                });
                self.session = Some(SessionHandle { tx, task });

                let _ = self
                    .event_tx
                    .send(UiEvent::ConnectionStatus("Connected!".into()));
            }
            Err(error) => {
                let _ = self
                    .event_tx
                    .send(UiEvent::ConnectionStatus(format!("Error: {error}")));
            }
        }
    }

    async fn disconnect(&mut self) {
        todo!();
    }

    async fn send(&mut self, packet: ControlPacket) {
        todo!();
    }
}

async fn run_session(
    stream: TcpStream,
    mut rx: mpsc::Receiver<SessionCommand>,
    event_tx: UnboundedSender<UiEvent>,
    audio_tx: crossbeam_channel::Sender<AudioCommand>,
) {
    let peer = stream.peer_addr().ok();
    let (mut reader, mut writer) = stream.into_split();

    let reader_task = async {
        loop {
            match read_packet(&mut reader).await {
                Ok(packet) => {
                    handle_inbound_packet(packet, &event_tx, &audio_tx).await;
                }
                Err(error) => {
                    tracing::warn!(?peer, ?error, "control reader ended");
                    break;
                }
            }
        }
    };

    let writer_task = async {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                SessionCommand::Send(packet) => {
                    if let Err(error) = write_packet(&mut writer, &packet).await {
                        tracing::warn!(?peer, ?error, "control writer ended");
                        break;
                    }
                }
                SessionCommand::Shutdown => break,
            }
        }
    };

    tokio::select! {
        _ = reader_task => {},
        _ = writer_task => {},
    }

    let _ = event_tx.send(UiEvent::ConnectionLost);
}

async fn write_packet<W>(writer: &mut W, packet: &ControlPacket) -> std::io::Result<()>
where
    W: AsyncWriteExt + Unpin,
{
    let payload = encode_packet(packet);
    writer.write_u32_le(payload.len() as u32).await?;
    writer.write_all(&payload).await?;
    writer.flush().await?;
    Ok(())
}

async fn read_packet<R>(reader: &mut R) -> std::io::Result<ControlPacket>
where
    R: AsyncReadExt + Unpin,
{
    let len = reader.read_u32_le().await? as usize;
    let mut buf = AlignedVec::with_capacity(len);
    reader.read_exact(&mut buf).await?;
    Ok(decode_packet(&buf))
}
