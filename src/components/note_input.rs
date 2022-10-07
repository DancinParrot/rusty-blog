use yew::prelude::*;
use crate::components::note::Note;

#[derive(Clone, Properties, PartialEq)]
pub struct NoteDetailsProps {
    pub note: Note,
    pub on_click: Callback<Note>,
}

// Take in an instance of note struct as parameter
// Another section to display note content
#[function_component(NoteInput)]
pub fn note_input(props: &NoteDetailsProps) -> Html {

    let on_click = props.on_click.clone();

    let on_note_submit = {
                let on_click = on_click.clone();
                let note = props.note.clone();
                Callback::from(move |_| {
                    on_click.emit(note.clone())
                })
    };

    let note: Note = props.note.clone();

    html! {
        <div class="sm:z-1">
            <div class="flex flex-col">
                <div class="flex justify-between">
                    <input type="text" class="text-4xl" value={ note.title } onchange={on_note_submit} />
                    <div class="flex space-x-2">
                        <button class="inline-block rounded border border-[#2A9D8F] bg-[#2A9D8F] px-12 py-3 text-sm font-medium text-white hover:bg-transparent hover:text-[#2A9D8F] focus:outline-none focus:ring active:text-[#2A9D8F]">{ "Save" }</button> // onclick will send an api request to update note
                        <button class="inline-block rounded border border-[#2A9D8F] px-12 py-3 text-sm font-medium text-[#2A9D8F] hover:bg-[#2A9D8F] hover:text-white focus:outline-none focus:ring active:bg-[#2A9D8F]">{ "Delete" }</button> // onclick will send api request to delete note
                    </div>
                </div>
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
                <div class="">
                    <div id="editor"></div>
                </div>
            </div>
        </div>
    }
}