use yew::prelude::*;
use yew_router::prelude::*;

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <Header />
                <Switch<AppRoute> render={Switch::render(switch)} />
                <Footer />
            </BrowserRouter>
        </UserContextProvider>
    }
}