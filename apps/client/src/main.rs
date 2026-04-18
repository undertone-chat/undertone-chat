mod components;
use dioxus::prelude::*;

const ICON: Asset = asset!("/icons/icon.ico");
const LOGO: Asset = asset!("/assets/undertone_icon_transparent.svg");
const MAIN_CSS: Asset = asset!("/assets/style/main.css");
// const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Launching dioxus app");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: ICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        // document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div{
            class: "w-[50cqi] m-auto",
            img { src: LOGO}
        }
    }
}
