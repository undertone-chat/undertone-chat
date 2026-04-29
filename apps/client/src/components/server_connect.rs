// use crate::{components::Input, control_connection::ControlCommand};
use dioxus::prelude::*;
// use tokio::sync::mpsc;
#[component]
pub fn ServerConnect() -> Element {
    let mut server_addr = use_signal(|| "".to_string());
    let mut server_port = use_signal(|| "".to_string());
    // let mut warning = use_signal(|| None::<String>);

    // let control_tx = consume_context::<mpsc::UnboundedSender<ControlCommand>>();

    let on_connect_pressed = move |_| {
        tracing::debug!("Connect pressed");
        // let _ = control_tx.send(ControlCommand::Connect("127.0.0.1:9990".to_string()));
    };
    let on_disconnect_pressed = move |_| {};

    rsx! {
        div {
            div { class: "input-row",
                input {
                    r#type: "text",
                    name: "server_addr",
                    placeholder: "127.0.0.1",

                    oninput: move |e: FormEvent| { server_addr.set(e.value()) },

                    value: server_addr,
                }
                input {
                    r#type: "text",
                    oninput: move |e: FormEvent| server_port.set(e.value()),
                    placeholder: "9990",
                    value: server_port,
                }
            }
            div { class: "button-row",
                button { onclick: on_connect_pressed, "Connect" }
                button { onclick: on_disconnect_pressed, "Disconnect" }
            }
            // if let Some(msg) = warning() {
            //     p { "{ msg }" }
            // }
        }
    }
}

pub fn validate_ip_addr(addr: &str) -> bool {
    !addr.is_empty()
}
