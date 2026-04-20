mod components;
mod audio;
mod control_connection;
use dioxus::prelude::*;
use tracing::Level;
use audio::{ AudioCommand};

use crossbeam_channel::unbounded;
use components::ServerConnect;
use tokio::sync::mpsc;

use crate::control_connection::{ControlCommand, ControlEvent};

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
    let mut is_muted = use_signal(|| false);

    // Spawn the Audio thread ONLY ONCE using use_hook
    use_hook(|| {
        // Create channels for communication
        let (control_tx, control_rx) = mpsc::unbounded_channel::<ControlCommand>();
        let (event_tx, mut event_rx) = mpsc::unbounded_channel::<ControlEvent>();
        let (audio_tx, audio_rx) = unbounded();

        provide_context(control_tx);
        // Provide the control sender to the whole dioxus app.
        std::thread::Builder::new().name("undertone-audio".into())
            .spawn(move || {
                audio::AudioEngine::new(audio_rx).run();
            })
        .expect("Failed to spawn audio thread");

        tokio::spawn(async move {
            control_connection::run_tcp_actor(control_rx, event_tx, audio_tx).await;
        });

        // Bridge to listen for events.
        spawn(async move {
            while let Some(event) = event_rx.recv().await {
                match event {
                    ControlEvent::StatusChanged(message) => connection_status.set(message),
                    ControlEvent::ConnectionLost => connection_status.set("Disconnected".to_string()),
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

               // document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div{ "Welcome to the shit" }
        div{
            img { src: LOGO, class: "logo"}
        }
        ServerConnect {}
        div { "Status: {connection_status}"}
    }
}
