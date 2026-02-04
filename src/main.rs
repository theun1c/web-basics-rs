mod logger;
mod components; // берут названия с папок в которых лежит mod 
use yew::prelude::*;
use components::main_title::MainTitle; // обращаемся к подключенному моду
// обращаемся к названию файла этого мода и к компоненту в этом файле
use components::text_fields::input_field::InputField;
use components::buttons::basic_button::BasicButton;
use components::cards::colored_card::ColoredCard;

#[component]
fn App() -> Html {


    html! {
        <>
            <MainTitle/>

            <div class="input_row_container">
                <InputField/>
                <BasicButton/>
            </div>

            <div class="container" style="margin-top: 1.5rem;">
                <ColoredCard bg_color="#a8e3b3" bgb_color="#2f9646"/>
            </div>

            <div class="container" style="margin-top: 1.5rem;">
                <ColoredCard bg_color="#eedc92" bgb_color="#e08405"/>
            </div>
            
            <div class="container" style="margin-top: 1.5rem;">
                <ColoredCard bg_color="#eebcbe" bgb_color="#d13032"/>
            </div>

        </>
    }
}

//
// entry point
fn main() { 
    yew::Renderer::<App>::new().render();
}
