use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Callback<()>,
}

#[function_component(ButtonNew)]
pub fn button_new(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });
    html! {
        <button onclick={button_onclick}>{&props.label}</button>
    }
}