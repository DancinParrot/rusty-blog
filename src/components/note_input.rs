use std::ops::Deref;
use yew::prelude::*;
use crate::components::note::Note;
use crate::components::note_form::text_input::TextInput;
use crate::components::note_form::textarea::Textarea;

#[derive(Clone, Properties, PartialEq)]
pub struct NoteDetailsProps {
    pub note: Note,
    pub on_click: Callback<Note>,
}

// Take in an instance of note struct as parameter
// Another section to display note content
#[function_component(NoteInput)]
pub fn note_input(props: &NoteDetailsProps) -> Html {

    // Set states for persistency
    let title_state = use_state(|| None);
    let content_state = use_state(|| None);

    let note = props.note.clone();

    // let on_click = props.on_click.clone();

    // let on_note_submit = {
    //             let on_click = on_click.clone();
    //             let note = props.note.clone();
    //             Callback::from(move |_| {
    //                 on_click.emit(note.clone())
    //             })
    // };
    

    // Handle text input for note title

    let title_changed = {
        let title_state = title_state.clone();

        Callback::from(move |title| {
            title_state.set(Some(title))
        })
    };

    // Handle text input for note content
    let content_changed = {
        let content_state = content_state.clone();

        Callback::from(move |content| {
            content_state.set(Some(content))
        })
    };

    // Handle save button event

    // Handle delete button event

    // Update tags vector

    html! {
        <div class="sm:z-1">
            <form class="flex flex-col">
                <div class="sm:grid sm:grid-cols-2">
                    <TextInput 
                    value={ 
                        if title_state.deref().is_some() { 
                            title_state.deref().clone().unwrap()
                        } else {
                            note.title.clone()
                        }
                    } 
                    on_change={ title_changed } />
                    // <input type="text" class="text-4xl" value={ note.title } onchange={on_note_submit} />
                    <div class="flex justify-end space-x-2">
                        <button class="inline-block rounded border border-[#2A9D8F] bg-[#2A9D8F] px-12 py-3 text-sm font-medium text-white hover:bg-transparent hover:text-[#2A9D8F] focus:outline-none focus:ring active:text-[#2A9D8F]">{ "Save" }</button> // onclick will send an api request to update note
                        <button class="inline-block rounded border border-[#2A9D8F] px-12 py-3 text-sm font-medium text-[#2A9D8F] hover:bg-[#2A9D8F] hover:text-white focus:outline-none focus:ring active:bg-[#2A9D8F]">{ "Delete" }</button> // onclick will send api request to delete note
                    </div>
                </div>
                <div class="mt-4">
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
                <div class="mt-4">
                    <Textarea 
                    value={  
                        if content_state.deref().is_some() { 
                            content_state.deref().clone().unwrap()
                        } else {
                            note.content.clone()
                        }
                    }
                    on_change = { content_changed } />
                </div>
            </form>
        </div>
    }
}
