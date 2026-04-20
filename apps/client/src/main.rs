// Modules
mod audio;
mod components;
mod control_connection;
mod ui_events;
// Dependencies
use components::ServerConnect;
use crossbeam_channel::unbounded;
use dioxus::prelude::*;
use tokio::sync::mpsc;
use tracing::Level;
use ui_events::UiEvent;

use crate::control_connection::{ControlCommand, ControlConnection};

const ICON: Asset = asset!("/icons/icon.ico");
const LOGO: Asset = asset!("/assets/undertone_icon.svg");
const MAIN_CSS: Asset = asset!("/assets/style/main.css");
const BASE_CSS: Asset = asset!("/assets/style/base.css");
const COMPONENT_CSS: Asset = asset!("/assets/style/dx-components-theme.css");
const FONT_CSS: Asset = asset!("/assets/style/fonts.css");

fn main() {
    dioxus::logger::init(Level::DEBUG).expect("failed to init logger");
    tracing::info!("Launching dioxus app");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Shared UI State
    let mut connection_status = use_signal(|| "Disconnected".to_string());

    // Spawn the Audio thread ONLY ONCE using use_hook
    use_hook(|| {
        // Create channels for communication
        let (control_tx, control_rx) = mpsc::unbounded_channel::<ControlCommand>();
        let (event_tx, mut event_rx) = mpsc::unbounded_channel::<UiEvent>();
        let (audio_tx, audio_rx) = unbounded();

        provide_context(control_tx);
        // Provide the control sender to the whole dioxus app.
        std::thread::Builder::new()
            .name("undertone-audio".into())
            .spawn(move || {
                audio::AudioEngine::new(audio_rx).run();
            })
            .expect("Failed to spawn audio thread");

        // Spawn control thread for incoming and outgoing control connection stuff.
        tokio::spawn(async move {
            ControlConnection::new(control_rx, event_tx, audio_tx)
                .run()
                .await;
            // control_connection::run_tcp_actor(control_rx, event_tx, audio_tx).await;
        });

        // Bridge to listen for events.
        spawn(async move {
            while let Some(event) = event_rx.recv().await {
                match event {
                    UiEvent::ConnectionStatus(message) => connection_status.set(message),
                    UiEvent::ConnectionLost => connection_status.set("Disconnected".to_string()),
                    _ => {}
                }
            }
        })
    });
    rsx! {
        document::Link { rel: "icon", href: ICON }
        document::Link { rel: "stylesheet", href: FONT_CSS}
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: BASE_CSS}
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }

        div{ "Welcome to the shit" }
        div{
            img { src: LOGO, class: "logo"}
        }
        ServerConnect {}
        div { "Status: {connection_status}"}
    }
}
