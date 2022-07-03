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


    log!(serde_json::to_string_pretty(&test).unwrap());
    html! {
        <h1>{"Hello!"}</h1>
    }
}
