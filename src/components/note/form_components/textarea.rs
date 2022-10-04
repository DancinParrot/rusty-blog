use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(Textarea)]
pub fn textarea(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });
    html! {
        <textarea rows="8" content={props.content.clone()} onchange={onchange} />
    }   
}