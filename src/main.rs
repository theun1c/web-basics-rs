mod logger;
use yew::prelude::*;


// 
// Component for main title 
#[component]
fn MainTitle() -> Html {
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

//
// component for text field
#[component]
fn InputField() -> Html {
    html! {
        <>
            <div class="container">
                <input type="text" placeholder="Book / Author / Pages / Time" class="text-field_main"/>
            </div>
        </>
    }
}

#[component]
fn App() -> Html {

    html! {
        <>
            <MainTitle/>
            <InputField/>
        </>
    }
}


fn main() { 
    yew::Renderer::<App>::new().render();
}
