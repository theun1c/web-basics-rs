use yew::prelude::*;

// 
// Component for main title 
#[component]
pub fn MainTitle() -> Html {
    html! {
        <>
            <div class="container">     
                <div class="main-title_text">
                    {"\"Hello, World!\""}
                </div>
            </div>
        </>
    }
}