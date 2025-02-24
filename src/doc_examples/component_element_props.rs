// ANCHOR: all
#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    // ANCHOR: Clickable_usage
    rsx! {
        Clickable {
            href: "https://www.youtube.com/watch?v=C-M2hs3sXGo",
            body: rsx!("How to " i { "not" } " be seen")
        }
    }
    // ANCHOR_END: Clickable_usage
}

// ANCHOR: Clickable
#[derive(PartialEq, Clone, Props)]
struct ClickableProps {
    href: String,
    body: Element,
}

fn Clickable(props: ClickableProps) -> Element {
    rsx!(
        a { href: "{props.href}", class: "fancy-button", {props.body} }
    )
}
// ANCHOR_END: Clickable
