use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(ErrorPage)]
pub fn error_page() -> Html {
    html! {
        <section class="bg-white">
            <div class="py-8 px-4 mx-auto h-screen max-w-screen-xl flex justify-content-center items-center lg:py-16 lg:px-6">
                <div class="mx-auto max-w-screen-sm text-center">
                    <p class="mb-4 text-3xl tracking-tight font-bold md:text-4xl text-[#AC1E23]">{ "404" }</p>
                    <h1 class="mb-4 text-5xl tracking-tight font-extrabold lg:text-7xl">{ "Page not found." }</h1>
                    <p class="mb-3 text-lg font-light text-gray-500 dark:text-gray-400">{ "Sorry, we couldn't find what you're looking for." }</p>
                    <a class="inline-flex text-white bg-[#AC1E23] hover:bg-[#AC1E23]/75 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:focus:ring-primary-900 my-4">
                    <Link<Route> to={Route::Index}>{ "Back to Homepage" }</Link<Route>></a>
                </div>   
            </div>
        </section>
    }
}