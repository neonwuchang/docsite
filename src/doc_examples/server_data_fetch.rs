#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

fn main() {
    launch(App)
}

fn App() -> Element {
    let mut count = use_resource(get_server_data);

    rsx! {"server data is {count.value():?}"}
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    // Access a database
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok("Hello from the server!".to_string())
}
