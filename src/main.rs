use yew::prelude::*;
use yew_router::prelude::*;

mod components; // component name
use components::note::Note;
use components::error::ErrorPage;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Note,
    #[not_found]
    #[at("/404")]
    ErrorPage,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Note => html! {
            <Note content="hello" />
        },
        Route::ErrorPage => html! {
            <ErrorPage />
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
