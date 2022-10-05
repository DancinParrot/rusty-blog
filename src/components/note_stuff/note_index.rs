use serde::Deserialize;
use serde::Serialize;
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;
use yew::prelude::*;
use gloo_console::log;
use crate::components::note::note_form::NoteForm;
use crate::components::note::note_form::Data;
use crate::components::navbar::Navbar;
use chrono::{DateTime, Utc};
use wasm_bindgen_futures;
use reqwasm::http::*;

// #[serde_with::serde_as]
// #[derive(Properties, PartialEq, Serialize, Deserialize)]
// pub struct Task {
//     pub note_id: String,
//     pub content: String,
//     #[serde_as(as = "serde_with::DurationSeconds<i64>")]
//     pub date_created: DateTime<Utc>,
//     #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
//     pub date_updated: DateTime<Utc>,
//     pub title: String,
//     pub notebook_id: String 
// }

// pub struct TaskResponse {
    // pub data: Vec<Task>,
// }

#[function_component(NoteIndex)]
pub fn note_index() -> Html {
    let form_submit = Callback::from(|data: Data| {
        log!("form content is", data.content);
    });

    // let onload = {
    //     Callback::from(move |_| {
    //         wasm_bindgen_futures::spawn_local(async move {
    //                 let response = Request::get("http://localhost:8080/get-all-notes")
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .json::<TaskResponse>()
    //                 .await
    //                 .unwrap();
    //                 log!(serde_json::to_string_pretty(&response).unwrap());
    //         })
    //     })
    // };

    html! {
        <div class="">
            // List all notes
            //<Navbar />
            <NoteForm onsubmit={form_submit} />
        </div>
    }
}