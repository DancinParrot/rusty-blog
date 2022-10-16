use yew::prelude::*;
use reqwasm::http::Request;
use crate::components::note::Note;

use self::api_error::ApiError;

pub mod constants;
pub mod api_error;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NoteResponse {
    pub data: Vec<Note>
}

impl NoteResponse {

    pub async fn get_all_notes() -> Result<NoteResponse, ApiError> {
        let request = Request::get(&format!("{}/get-all-notes", constants::BACKEND_URL))
            .send()
            .await
            .unwrap();
        
        if request.ok() {
            // Return as a Result enum
            Ok(request.json::<NoteResponse>().await.unwrap())
        } else {
            match request.status() {
                501 => Err(ApiError::GatewayError),
                _ => Err(ApiError::Unknown)
            }
        }
    }

    // Take in mutable reference of note
    pub fn get_note(note: &mut Note) {

    }

    pub fn update_note(note: &Note) -> bool {
        false
    }

    // Returns true on success and vice versa
    pub fn delete_note(note: &Note) -> bool {
        false
    }
}