use dioxus::signals::WritableExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{ UnboundedSender,UnboundedReceiver };
use tokio::io::AsyncWriteExt;
use crossbeam_channel::Sender as CrossbeamSender;
use dioxus::prelude::Signal;

use crate::audio::AudioCommand;

#[derive(Debug)]
pub enum ControlCommand {
    Connect(String),
    Join(u32),
    Disconnect,
}

pub enum ControlEvent {
    StatusChanged(String),
    ConnectionLost,
}

pub async fn run_tcp_actor(
     mut cmd_rx: UnboundedReceiver<ControlCommand>,
     event_tx: UnboundedSender<ControlEvent>, // UI Listener
     audio_tx: CrossbeamSender<AudioCommand>,
     ) {
     let mut stream: Option<TcpStream> = None;

     // This loop stays alive independently of the UI components
     while let Some(cmd) = cmd_rx.recv().await {
         match cmd {
             ControlCommand::Connect(address) => {
                 tracing::debug!("Got Control Command Connect");
                 let _ = event_tx.send(ControlEvent::StatusChanged("Connecing...".into()));
                                  match TcpStream::connect(&address).await {
                     Ok(tcp) => {
                         let _ = event_tx.send(ControlEvent::StatusChanged("Connected!".into()));
                         stream = Some(tcp);
                         let _ = audio_tx.send(AudioCommand::StartStreaming(address));
                     }
                     Err(e) => {
                         let _ = event_tx.send(ControlEvent::StatusChanged(format!("Error: {}", e)));
                     }
                 }
             }
             ControlCommand::Disconnect => {
                 stream = None;
                 let _ = event_tx.send(ControlEvent::ConnectionLost);
                 let _ = audio_tx.send(AudioCommand::Shutdown);
             }
             _ => {
                 tracing::debug!("Unhandled Command Message!");
             }
         }
     }
 }
