mod components;
use dioxus_native::prelude::*;

const ICON: Asset = asset!("/icons/icon.ico");
const LOGO: Asset = asset!("/assets/undertone_icon.svg");
const MAIN_CSS: Asset = asset!("/assets/style/main.css");
const COMPONENT_CSS: Asset = asset!("/assets/style/dx-components-theme.css");
const FONT_CSS: Asset = asset!("/assets/style/fonts.css");
fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Launching dioxus app");
    dioxus_native::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: ICON }
        document::Link { rel: "stylesheet", href: FONT_CSS}
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }
               // document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div{ "Welcome to the shit" }
        div{
            img { src: LOGO, class: "logo"}
        }
    }
}
