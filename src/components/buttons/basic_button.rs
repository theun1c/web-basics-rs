use yew::prelude::*;

//
// button component 
#[component]
pub fn BasicButton() -> Html {
    html! {
        <>
            <button class="">
                {"Push me"}
            </button>
        </>
    }
}