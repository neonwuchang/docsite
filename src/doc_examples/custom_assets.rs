use dioxus::prelude::*;
use manganis::mg;

fn main() {
    launch(app);
}

fn App() -> Element {
    rsx! {
        div { img { src: mg!(file("public/static/scanner.png")) } }
    }
}
