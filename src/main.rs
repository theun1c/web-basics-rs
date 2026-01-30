mod logger;
mod components; // берут названия с папок в которых лежит mod 
use yew::prelude::*;
use components::main_title::MainTitle; // обращаемся к подключенному моду
// обращаемся к названию файла этого мода и к компоненту в этом файле
use components::input_field::InputField;

#[component]
fn App() -> Html {

    html! {
        <>
            <MainTitle/>

            <InputField/>
        </>
    }
}

//
// entry point
fn main() { 
    yew::Renderer::<App>::new().render();
}
