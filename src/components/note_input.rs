use yew::prelude::*;
use crate::note::Note;

#[derive(Clone, Properties, PartialEq)]
struct NoteDetailsProps {
    note: Note,
}

// Another section to display note content
#[function_component(NoteInput)]
fn note_input(props: &NoteDetailsProps) -> Html {
    html! {
        <div class="z-40  sm:z-1">
           <div class="flex justify-between">
                <input type="text" onchange={} />
                <div class="flex">
                    <button></button> // onclick will send an api request to update note
                    <button></button> // onclick will send api request to delete note
                </div>
           </div> 

           input
        </div>
    }
}