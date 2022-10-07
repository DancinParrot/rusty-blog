use yew::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, Debug)]
pub struct Note {
    pub note_id: String,
    pub content: String,
    pub title: String,
    pub tags: Vec<String>, 
}

struct Tag {
    pub tag_id: String,
    pub name: String,
    pub on_click: Callback<Tag>,
}

struct NoteTags {
    pub note_id: String,
    pub tag_id: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct NoteListProps {
    pub notes: Vec<Note>,
    pub on_click: Callback<Note>, // If click, notify of selected note
}

// Create a product card for each instance of note in note list
#[function_component(NoteList)]
pub fn note_list(NoteListProps { notes, on_click }: &NoteListProps) -> Html {
    let on_click = on_click.clone();

    notes.iter().map(|note| {
        let on_note_select = {
            let on_click = on_click.clone();
            let note = note.clone();
            Callback::from(move |_| {
                on_click.emit(note.clone())
            })
        };

        html! {
            <a
            class="block p-8 bg-white mb-4"
            onclick={on_note_select}
            >
                <span
                    class="rounded-full bg-green-100 px-3 py-1.5 text-xs font-medium text-green-600"
                >
                    { "4.3" }
                </span>

                <div class="mt-4 text-gray-500 sm:pr-8">
                    <h2 class="mt-4 text-2xl text-[#264653]">{format!("{}", note.title)}</h2>
                    <div>
                        {
                            note.tags.iter().map(|tag| {
                                html! {
                                    <span
                                        class="rounded-full bg-gray px-3 py-1.5 text-xs font-medium text-black"
                                    >
                                        { format!("{}", tag) } 
                                    </span>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <p class="mt-2 text-sm">{ if note.content.len() > 50 { format!("{}", &note.content[..50]) } else { format!("{}", &note.content) } }</p>
                </div>
            </a>
        }
    }).collect()

}

impl Note {

    pub fn get_all_notes(notes: &Vec<Note>) {

    }

    // Take in mutable reference of note
    pub fn get_note(note: &mut Note) {

    }

    pub fn update_note(note: &Note) -> bool {
        false
    }

    // Returns true on success and vice versa
    pub fn delete_note(note: &Note) -> bool {
        false
    }
}