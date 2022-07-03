use serde::{Deserialize, Serialize};
use yew::prelude::*;
use gloo::console::log;
use stylist::{yew::styled_component, style};

#[derive(Serialize, Deserialize )]
struct Test {
    username: String,
    lang: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let name = String::from("Paweł");
    let test = Test{
        username: name,
        lang: "Rust".to_owned()
    };

    let styles = style!(
        r#"
        background-color: #000000;
        color: #ffffff;
        "#
    ).expect("Cannot render styles!");

    let message: Option<&str> = Some("This is a message for you!");

    let tasks: Vec<&str> = vec!["make a dinner", "hate PHP"];

    log!(serde_json::to_string_pretty(&test).unwrap());
    html! {
        <div class={styles}>
            <h1>{"Hello!"}</h1>
            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"No message for you!"}</p>
            }
            <ul>
                {list_to_html(tasks)}
            </ul>
        </div>
    }
}


fn list_to_html<T: std::fmt::Display>(list: Vec<T>) -> Vec<Html> {
    list.iter().map(|task| html! {<li>{task}</li>}).collect()
}
