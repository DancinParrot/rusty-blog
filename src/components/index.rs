use yew::prelude::*;
use crate::components::note::*;

// Root element to display all components
#[function_component(Index)]
pub fn index() -> Html {
    let notes: Vec<Note> = vec![
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
            tags: vec![String::from("tag1"), String::from("tag3")],
        }
    ];

    let selected_note = use_state(|| None);

    let handle_onclick = {
        let selected_note = selected_note.clone();

        Callback::from(move |note: Note| {
            selected_note.set(Some(note))
        }
    )};



    html! {
        <section>
            <div class="h-screen sm:grid sm:grid-cols-5">
                <div class="h-full sm:col-span-2 bg-gray-200 px-4 py-4">
                    <h1 class="text-2xl my-5">{ "Notebook" }</h1>
                    <NoteList notes={notes} on_click={handle_onclick.clone()} /> 
                </div>
                <div class="sm:col-span-3">

                </div>
            </div>
        </section>
    }
}