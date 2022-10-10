use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: Option<String>,
    pub on_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {

    let on_change = props.on_change.clone();

    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let input =  target.unchecked_into::<HtmlInputElement>().value();
        on_change.emit(input.clone());        
    });

    let value = props.value.clone().unwrap_or_default();

    html! {
        <input type="text" class="text-4xl" {onchange} {value} />       
    }
}