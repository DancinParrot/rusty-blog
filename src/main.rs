use yew::prelude::*;
use yew_router::prelude::*;

mod components; // component name
use components::test::RenderedAt;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/test")]
    RenderedAt,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <h1>{ "Home" }</h1>
        },
        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
        Route::RenderedAt => html! {
            <RenderedAt />
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
