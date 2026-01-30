use yew::prelude::*;

//
// component for input field
//
// TODO: You can enter from which page to which page you read
// like 10-23 - This means that you have read 13 pages. 
//
// TODO: Also, if you entered an empty time, you can enter a default value based on past readings
// (you just need to find out how long it takes a person to read 1 page on average)
// component for text field
#[component]
pub fn InputField() -> Html {
    html! {
        <>
            <div class="container">
                <input type="text" class="text-field_main" placeholder="Book / Author / Pages / Time"/>
            </div>
            
        </>
    }
}