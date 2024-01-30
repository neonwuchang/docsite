#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

#[derive(serde::Deserialize, Debug)]
struct ApiResponse {
    #[serde(rename = "message")]
    image_url: String,
}

pub fn App() -> Element {
    // ANCHOR: use_future
    let future = use_future((), |_| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });
    // ANCHOR_END: use_future

    // ANCHOR: render
    cx.render(match future.value() {
        Some(Ok(response)) => rsx! {
            button {
                onclick: move |_| future.restart(),
                "Click to fetch another doggo"
            }
            div {
                img {
                    max_width: "500px",
                    max_height: "500px",
                    src: "{response.image_url}",
                }
            }
        },
        Some(Err(_)) => rsx! { div { "Loading dogs failed" } },
        None => rsx! { div { "Loading dogs..." } },
    })
    // ANCHOR_END: render
}

#[component]
fn RandomDog(breed: String) -> Element {
    // ANCHOR: dependancy
    let future = use_future((breed,), |(breed,)| async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });
    // ANCHOR_END: dependancy

    cx.render(rsx!(()))
}
