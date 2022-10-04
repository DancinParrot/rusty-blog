use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::note::note_index::NoteIndex;
use crate::components::error::ErrorPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    NoteIndex,
    #[not_found]
    #[at("/404")]
    ErrorPage,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::NoteIndex => html! {
            <NoteIndex />
        },
        Route::ErrorPage => html! {
            <ErrorPage />
        }
    }
}