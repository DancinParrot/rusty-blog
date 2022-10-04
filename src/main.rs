use yew::prelude::*;
use yew_router::prelude::*;

mod components; // declare component here to index
mod router;
use router::{switch, Route};

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
