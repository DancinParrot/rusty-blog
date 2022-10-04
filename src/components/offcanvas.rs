
use yew::prelude::*;

#[function_component(Offcanvas)]
pub fn Offcanvas() -> Html {
    html! {
        <section class="fixed inset-y-0 right-0 z-50 flex">
            <div class="w-screen max-w-sm">
                <div class="flex h-full flex-col divide-y divide-gray-200 bg-[#2A9D8F]">
                    <div class="overflow-y-scroll"> 
                        <header class="flex h-16 items-center justify-between pl-6">
                        <span class="text-sm font-medium uppercase tracking-widest">
                            Menu
                        </span>
                    </div>
                </div>
            </div>
        </section>
    }
}