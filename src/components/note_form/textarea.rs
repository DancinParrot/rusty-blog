use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: Option<String>, // Display existing content to user
    pub on_change: Callback<String>, // To pass input back
}

#[function_component(Textarea)]
pub fn textarea(props: &Props) -> Html {


    let state = use_state(String::new);
    let is_loaded = use_state(|| false);

    let on_change = props.on_change.clone();

    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let input =  target.unchecked_into::<HtmlInputElement>().value();
        on_change.emit(input.clone());        
    });

    let value = props.value.clone().unwrap_or_default();

    html! {
        <textarea rows="10" type="text" class="w-full text-gray-800 text-2xl" {value} onchange={onchange} />       
    }
}