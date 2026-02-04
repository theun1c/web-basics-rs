use yew::prelude::*;
use yew_autoprops::autoprops;


#[autoprops]
#[component]
pub fn ColoredCard(
    #[prop_or_default]
    bg_color: &AttrValue,
    #[prop_or_default]
    bgb_color: &AttrValue,
) -> Html {
    let card_style = format!("background-color: {}; border-color: {}", bg_color, bgb_color);
    html! {
        // needs to be aligned inside the card
        <div class="colored-card" style={card_style}>
    
            <div class="colored-card_content">

                <div class="colored-card_content_top">
                    <span class="name-author">{"Name / Author"}</span>
                </div>

                <div class="colored-card_content_bottom">
                    <span class="time-pages">{"time / page count"}</span>
                    <span class="date">{"date dd/mm/yy"}</span>
                </div>

            </div>

        </div>
    }
} 