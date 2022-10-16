use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::{prelude::*, html::IntoPropValue};
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: Option<String>, // Display existing content to user
    pub on_change: Callback<String>, // To pass input back
}

#[function_component(Textarea)]
pub fn textarea(props: &Props) -> Html {

    let state = use_state(String::new);
    let is_loaded = use_state(|| false);


    let onchange = {
        let on_change = props.on_change.clone();
        let state = state.clone();

        Callback::from(move |event: Event| {
            let target = event.target().unwrap();
            let input =  target.unchecked_into::<HtmlTextAreaElement>().value();
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
            // if !*is_loaded && state.is_empty() && value.is_some() {
            //     log!("Onload note content: ", value.clone().unwrap());
            //     state.set(value.unwrap());
            //     is_loaded.set(true);
            // }

            // Load also, if value is different than the previous value (as this means the user selected another note)
            if !*is_loaded && state.is_empty() && value.is_some() {
                log!("Onload note content: ", value.clone().unwrap());
                state.set(value.unwrap());
                is_loaded.set(true);
            } 
            // else if state.as_str() != value {
            //     log!("Onload note content: ", value.clone().unwrap());
            //     state.set(value.unwrap());
            //     is_loaded.set(true);
            // }

            || {}
        })
    }

    html! {
        <textarea rows="10" type="text" class="w-full text-gray-800 text-2xl" {onchange} value={state.deref().clone()} />       
    }
}