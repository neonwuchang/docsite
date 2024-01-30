#![allow(non_snake_case)]
#![allow(unused)]

use dioxus::prelude::*;

pub fn App() -> Element {
    cx.render(rsx!(
        Empty {},
        Children {},
        Fragments {},
        Attributes {},
        VariableAttributes {},
        CustomAttributes {},
        Formatting {},
        Expression {},
    ))
}

pub fn Empty() -> Element {
    // ANCHOR: empty
    cx.render(rsx!(div {
        // attributes / listeners
        // children
    }))
    // ANCHOR_END: empty
}

pub fn Children() -> Element {
    // ANCHOR: children
    cx.render(rsx!(ol {
        li {"First Item"}
        li {"Second Item"}
        li {"Third Item"}
    }))
    // ANCHOR_END: children
}

pub fn Fragments() -> Element {
    // ANCHOR: fragments
    cx.render(rsx!(
        p {"First Item"},
        p {"Second Item"},
        Fragment {
            span { "a group" },
            span { "of three" },
            span { "items" },
        }
    ))
    // ANCHOR_END: fragments
}

pub fn ManyRoots() -> Element {
    // ANCHOR: manyroots
    cx.render(rsx!(
        p {"First Item"},
        p {"Second Item"},
    ))
    // ANCHOR_END: manyroots
}

pub fn Attributes() -> Element {
    // ANCHOR: attributes
    cx.render(rsx!(img {
        src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
        class: "primary_button",
        width: "10px"
    }))
    // ANCHOR_END: attributes
}

pub fn VariableAttributes() -> Element {
    // ANCHOR: variable_attributes
    let written_in_rust = true;
    let button_type = "button";
    cx.render(rsx!(button {
        disabled: "{written_in_rust}",
        class: "{button_type}",
        "Rewrite it in rust"
    }))
    // ANCHOR_END: variable_attributes
}

pub fn CustomAttributes() -> Element {
    // ANCHOR: custom_attributes
    cx.render(rsx!(div {
        "style": "width: 20px; height: 20px; background-color: red;",
    }))
    // ANCHOR_END: custom_attributes
}

pub fn Formatting() -> Element {
    // ANCHOR: formatting
    let coordinates = (42, 0);
    let country = "es";
    cx.render(rsx!(div {
        class: "country-{country}",
        left: "{coordinates.0:?}",
        top: "{coordinates.1:?}",
        // arbitrary expressions are allowed,
        // as long as they don't contain `{}`
        div {
            "{country.to_uppercase()}"
        },
        div {
            "{7*6}"
        },
        // {} can be escaped with {{}}
        div {
            "{{}}"
        },
    }))
    // ANCHOR_END: formatting
}

pub fn Expression() -> Element {
    // ANCHOR: expression
    let text = "Dioxus";
    cx.render(rsx!(span {
        text.to_uppercase(),
        // create a list of text from 0 to 9
        (0..10).map(|i| rsx!{ i.to_string() })
    }))
    // ANCHOR_END: expression
}

pub fn Loops() -> Element {
    // ANCHOR: loops
    cx.render(rsx! {
        // use a for loop where the body itself is RSX
        div {
            // create a list of text from 0 to 9
            for i in 0..3 {
                // NOTE: the body of the loop is RSX not a rust statement
                div {
                    "{i}"
                }
            }
        }
        // iterator equivalent
        div {
            (0..3).map(|i| rsx!{ div { "{i}" } })
        }
    })
    // ANCHOR_END: loops
}

pub fn IfStatements() -> Element {
    // ANCHOR: ifstatements
    cx.render(rsx! {
        // use if statements without an else
        if true {
            rsx!(div { "true" })
        }
    })
    // ANCHOR_END: ifstatements
}
