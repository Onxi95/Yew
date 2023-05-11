use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <button {onclick}>{"add 1"}</button>
            <p>{"count: "}{*counter}</p>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
