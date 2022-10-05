use chrono::{DateTime, Utc};
use yew::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Properties, PartialEq)]
pub struct NoteViewProps {
    // pub note_id: String,
    // pub content: String,
    // pub date_created: DateTime::<Utc>,
    // pub date_updated: DateTime::<Utc>,
    // pub title: String,
    // pub notebook_id: String 
    pub content: String,
}

#[function_component(Note)]
pub fn note(props: &NoteViewProps) -> Html {
    html! {
        <h1>{ &props.content }</h1>
    }
}
