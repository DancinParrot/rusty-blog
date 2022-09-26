use chrono::{DateTime, Utc};
use yew::prelude::*;
use reqwest::Error;

#[path = "../constants.rs"]
mod constants;

enum Blogs {

}

struct Blog {
    title: String,
    author: String,
    time: DateTime<Utc>,
    tags: Vec<String>,
    content: String
}

impl Blog {
    pub fn new(title: String, author: String, time: DateTime<Utc>, tags: Vec<String>, content: String) -> Blog {
        Blog { title, author, time, tags, content}
    }

    async fn getBlog() {
        let request_url = format!("{base}/collection/blogs", 
            base = constants::BACKEND_URL 
        );

        let response = reqwest::get(&request_url).await?;
    }
}

impl Component for Blog {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button class="hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p class="underline">{ self.value }</p>
            </div>
        }
    }
}

