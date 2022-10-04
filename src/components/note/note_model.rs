use yew::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Properties, PartialEq)]
pub struct NoteViewProps {
    pub note_id: String,
    pub content: String,
    pub date_created: DateTime::<Utc>,
    pub date_updated: DateTime::<Utc>,
    pub title: String,
    pub notebook_id: String 
}

