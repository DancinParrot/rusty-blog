use chrono::DateTime;
use yew::prelude::*;
use serde::{Serialize, Deserialize};

pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<Note, anyhow::Error>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct NoteViewProps {
    pub note_id: String,
    pub content: String,
    pub date_created: DateTime;
    pub date_updated: DateTime;
    pub title: String,
    pub notebook_id: String 
}

#[function_component(Note)]
pub fn note(props: &NoteViewProps) -> Html {
    html! {

    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Note {
    pub note_id: String,
    pub content: String,
    pub date_created: DateTime,
    pub date_updated: DateTime,
    pub title: String,
    pub notebook_id: String 
}

#[derive(Debug)]
pub struct FetchNotes {
    fetch_task: Option<FetchTask>,
    note: Option<Note>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl FetchNotes {
    fn view() {}
}


impl Component for FetchNotes {
    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            note: None,
            link,
            error: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        let request = match Request::get("http:localhost:8080/get-all-notes").body(Nothing) {
            Ok(result) => result,
            Error(err) => panic!("{}", err)
        };

        if request != None {
            return true
        }
    }
}