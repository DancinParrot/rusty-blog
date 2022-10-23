use yew::prelude::*;
use crate::components::note::*;
use crate::components::note_input::*;
use crate::components::api::NoteResponse;

// Root element to display all components
#[function_component(Index)]
pub fn index() -> Html {
    let mut notes: Vec<Note> = vec![
        Note {
            note_id: String::from("1"),
            content: String::from("Hello World!"),
            title: String::from("Note 1"),
            tags: vec![String::from("tag1"), String::from("tag3")],
        },
        Note {
            note_id: String::from("2"),
            content: String::from("Hello from Mars!"),
            title: String::from("Note 2"),
            tags: vec![String::from("tag1"), String::from("tag3")],
        }, 
        Note {
            note_id: String::from("3"),
            content: String::from("Hello from MarsHello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!!"),
            title: String::from("Note 3"),
            tags: vec![String::from("tag4"), String::from("tag5")],
        },
        Note {
                    note_id: String::from("3"),
                    content: String::from("Hello from MarsHello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!Hello from Mars!!"),
                    title: String::from("Note 3"),
                    tags: vec![String::from("tag4"), String::from("tag5")],
        }
    ];

    // Component state
    let is_loaded = use_state(|| false);

    // use_effect(move || {
    //     if !*is_loaded {
    //         let is_loaded = is_loaded.clone();

    //         wasm_bindgen_futures::spawn_local(async move {
    //             match NoteResponse::get_all_notes()
    //         })
    //     }
    // })

    let selected_note = use_state(|| None);
    let mut hidden: bool = true; 

    // Handle onclick events, will set the selected note variable
    let handle_onclick = {
        let selected_note = selected_note.clone();
        hidden = false;
        Callback::from(move |note: Note| {
            selected_note.set(Some(note))        
        }) 
    };

    // Use selected note to display its content in the NoteInput component
    let updated_note = use_state(|| None);

    let handle_onsubmit = {
        let updated_note = updated_note.clone();

        Callback::from(move |note: Note| {
            updated_note.set(Some(note))
        })
    };

    let note_details = selected_note.as_ref().map(|note| html! {
        <NoteInput note={note.clone()} on_click={handle_onsubmit} />
    });
   
    html! {
        <section>
            <div class="h-screen sm:grid sm:grid-cols-5">
                <div class="h-full sm:col-span-2 xl:col-span-1 bg-[#2A9D8F] px-4 py-4 overflow-auto">
                    <div class="grid grid-cols-2">
                        <h1 class="text-2xl my-5 text-white">{ "Notebook" }</h1>
                        <div class="flex justify-end items-center">
                            <button class="font-bold text-center text-4xl  rounded px-8 text-white">{"+"}</button>
                        </div>
                    </div>
                    <NoteList notes={notes} on_click={handle_onclick.clone()} /> 
                </div>
                <div hidden={hidden} class="sm:z-1 sm:block sm:col-span-3 xl:col-span-4 px-12 py-12">
                    {
                        if selected_note == use_state(|| None) {
                            html! {
                                <div class="w-full h-full flex justify-center items-center">
                                    <div class="text-center">
                                        <h2 class="text-2xl font-medium">{ "There's nothing here..." }</h2>
                                        <p class="mt-4 text-sm text-gray-500">
                                            { "Empty.. Try selecting or creating a new note! " }
                                        </p>

                                        <a
                                            href=""
                                            class="mt-8 inline-flex items-center rounded-lg bg-[#2A9D8F] px-5 py-3 font-medium text-white hover:bg-[#2A9D8F]/50"
                                        >
                                            { "Create a note" }
                                            <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke="currentColor"
                                            class="ml-3 h-4 w-4 flex-shrink-0"
                                            >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M14 5l7 7m0 0l-7 7m7-7H3"
                                            />
                                            </svg>
                                        </a>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {
                                for note_details
                            }
                        }
                    }
                </div>
            </div>
        </section>
    }
}