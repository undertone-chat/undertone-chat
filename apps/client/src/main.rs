mod components;
use dioxus::prelude::*;

use tracing::Level;

use crate::components::ServerConnect;

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
    }
}
