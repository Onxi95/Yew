use serde::{Deserialize, Serialize};
use yew::prelude::*;
use gloo::console::log;

#[derive(Serialize, Deserialize )]
struct Test {
    username: String,
    lang: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = String::from("Paweł");
    let test = Test{
        username: name,
        lang: "Rust".to_owned()
    };

    let message: Option<&str> = Some("This is a message for you!");


    log!(serde_json::to_string_pretty(&test).unwrap());
    html! {
        <>
            <h1>{"Hello!"}</h1>
            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"No message for you!"}</p>
            }
        </>
    }
}
