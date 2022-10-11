use std::ops::Deref;
use yew::prelude::*;
use crate::components::note::Note;
use crate::components::note_form::text_input::TextInput;
use crate::components::note_form::textarea::Textarea;
use yew_router::prelude::*;
use crate::router::Route;

use super::note;

#[derive(Clone, Properties, PartialEq)]
pub struct NoteDetailsProps {
    pub note: Note,
    pub on_click: Callback<Note>,
}

// Take in an instance of note struct as parameter
// Another section to display note content
#[function_component(NoteInput)]
pub fn note_input(props: &NoteDetailsProps) -> Html {

    // Set state for persistency
    let note_state = use_state(|| Note::default());

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
    let cloned_state = note_state.clone();

    let title_changed = Callback::from(move |title| {
        cloned_state.set(Note {
            title,
            ..cloned_state.deref().clone()
        })
    });

    // Handle text input for note content
    let cloned_state = note_state.clone();

    let content_changed = Callback::from(move |content| {
        cloned_state.set(Note {
            content,
            ..cloned_state.deref().clone()
        })
    });

    // Handle save button event

    // Handle delete button event

    // Update tags vector

    html! {
        <div class="sm:z-1">
            <form class="flex flex-col">
                <div class="sm:grid sm:grid-cols-2">
                    <TextInput value={ note.title } on_change={ title_changed } />
                    // <input type="text" class="text-4xl" value={ note.title } onchange={on_note_submit} />
                    <div class="flex justify-end space-x-2">
                        <button class="inline-block rounded border border-[#2A9D8F] bg-[#2A9D8F] px-12 py-3 text-sm font-medium text-white hover:bg-transparent hover:text-[#2A9D8F] focus:outline-none focus:ring active:text-[#2A9D8F]">{ "Save" }</button> // onclick will send an api request to update note
                        <button class="inline-block rounded border border-[#2A9D8F] px-12 py-3 text-sm font-medium text-[#2A9D8F] hover:bg-[#2A9D8F] hover:text-white focus:outline-none focus:ring active:bg-[#2A9D8F]">{ "Delete" }</button> // onclick will send api request to delete note
                    </div>
                </div>
                <div class="mt-4">
                    {
                        note_state.tags.iter().map(|tag| {
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
                    <Textarea value={ note.content } on_change = { content_changed } />
                </div>
            </form>
        </div>
    }
}