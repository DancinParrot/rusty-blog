use yew::{function_component, html, Properties};


#[function_component(RenderedAt)]
pub fn rendered_at() -> Html {
    html! {
        <p>
            <b>{ "Rendered at: " }</b>
        </p>
    }
}