use std::ops::Deref;

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

    // State for persistency (prevent reset on input bug)
    let state = use_state(String::new);
    let is_loaded = use_state(|| false);

    let onchange = {
        let on_change = props.on_change.clone();
        let state = state.clone();

        Callback::from(move |event: Event| {
            let target = event.target().unwrap();
            let input =  target.unchecked_into::<HtmlInputElement>().value();
            on_change.emit(input.clone());
            state.set(input);
        })
    };

    {
        let state = state.clone();
        let is_loaded = is_loaded.clone();
        let value = props.value.clone();
        
        // Check is component loaded, and if state is empty and if value got text
        // If loaded (cause is_loaded is set to false initally), set true for is_loaded, and state value
        use_effect(move|| {
            if !*is_loaded && state.is_empty() && value.is_some() {
                state.set(value.unwrap());
                is_loaded.set(true);
            }

            || {}
        })
    }


    html! {
        <input type="text" class="text-4xl" {onchange} value={state.deref().clone()} />       
    }
}