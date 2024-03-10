// ANCHOR: all
use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! { div { "Hello, world!" } }
}
// ANCHOR_END: all
