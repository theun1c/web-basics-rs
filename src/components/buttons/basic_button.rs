use yew::prelude::*;

//
// button component 
#[component]
pub fn BasicButton() -> Html {
    html! {
        <>
            <button class="basic-button">
                {"add"}
            </button>
        </>
    }
}