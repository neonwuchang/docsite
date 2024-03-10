#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "web")]
    dioxus_web::launch_cfg(app, dioxus_web::Config::new().hydrate(true));
    #[cfg(feature = "ssr")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
                axum::Server::bind(&addr)
                    .serve(
                        axum::Router::new()
                            .serve_dioxus_application("", ServeConfigBuilder::new(app, ()))
                            .into_make_service(),
                    )
                    .await
                    .unwrap();
            });
    }
}

fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}
