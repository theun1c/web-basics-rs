mod logger;
use once_cell::sync::OnceCell;
use logger::Logger;
use yew::prelude::*;
use std::{str, sync::Mutex};
use crate::logger::logger::get_logger;


#[derive(Clone, PartialEq)]
struct Person {
    id: u16, 
    name: AttrValue
}

#[component]
fn app() -> Html {
    let persons  = vec![
        Person{
            id: 1,
            name: "dima".into()
        },
        Person{
            id: 2,
            name: "dima2".into()
        }
    ];
    html! {
        <>
            <h1>{"Hello, world!"}</h1>
            <div>
                for person in &persons {
                    <p>{ format!("{} {}", person.id, person.name) }</p>
                }
            </div>
        </>
    }
}

fn main() { 

    #[cfg(not(target_arch = "wasm32"))]{
        let mut logger = get_logger().lock().unwrap();
        logger.success("запуск не wasm");
    }

    yew::Renderer::<app>::new().render();
}
