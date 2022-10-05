use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::index::Index;
use crate::components::error::ErrorPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    ErrorPage,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Index => html! {
            <Index />
        },
        Route::ErrorPage => html! {
            <ErrorPage />
        }
    }
}