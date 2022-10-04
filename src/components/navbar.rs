use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <section class="h-screen inset-y-0 right-0 z-50 flex">
            <div class="w-screen max-w-sm">
                <div class="flex h-full flex-col bg-[#2A9D8F]">
                    <header class="flex h-16 items-center justify-between pl-6">
                        <span class="text-lg font-medium font-bold text-white">
                            { "Hello World!" }
                        </span>
                            <button
                                aria-label="Close menu"
                                class="h-16 w-16 sm:hidden"
                                type="button"
                            >
                                <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="mx-auto h-5 w-5"
                                fill="white"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12"
                                />
                                </svg>
                            </button>
                    </header>
                    <nav class="flex flex-col text-sm font-medium text-white">
                        <a class="px-6 py-3"> { "Notebook 1" } </a>
                        <a class="px-6 py-3"> { "Notebook 2" } </a>
                    </nav>
                </div>
            </div>
        </section>
    }
}