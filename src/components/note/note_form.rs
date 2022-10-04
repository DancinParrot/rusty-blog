use std::clone;

use yew::prelude::*;
use chrono::{DateTime, Utc};
use gloo_console::log;
use crate::components::note::form_components::textarea::Textarea;
use crate::components::note::form_components::button_new::ButtonNew;

#[derive(Default, Clone)]
pub struct Data {
    pub content: String,
    pub date_update: DateTime::<Utc>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(NoteForm)]
pub fn note(props: &Props) -> Html {

    let state = use_state(|| Data::default());
    let cloned_state = state.clone();
    let content_changed = Callback::from(move |content| {
        cloned_state.set(
            Data {
                content,
                ..(*cloned_state).clone()
            });
    });

    let cloned_state = state.clone();
    let button_onclick = Callback::from(move |_| {
        cloned_state.set(
            Data {
                date_update: chrono::offset::Utc::now(),
                ..(*cloned_state).clone()
            });
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit: Callback<FocusEvent> = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = (*cloned_state).clone();
        form_onsubmit.emit(data);
    });

    html! {
        <form onsubmit={onsubmit}>
            <Textarea content="Hello!" handle_onchange={content_changed} />
            <ButtonNew label="New" onclick={button_onclick} />
            <p>{&state.date_update}</p>
            <p>{&state.content}</p>
        </form>
    }
}
