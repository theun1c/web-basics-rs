mod logger;
use once_cell::sync::OnceCell;
use logger::Logger;
use yew::prelude::*;
use std::{str, sync::Mutex};

static LOGGER: OnceCell<Mutex<Logger>> = OnceCell::new();

pub fn get_logger() -> &'static Mutex<Logger>{
    LOGGER.get_or_init(|| {
        let logger = Logger::new().expect("logger creating error");
        Mutex::new(logger)
    })
}

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
    // let mut logger = get_logger().lock().unwrap();
    // logger.error("Ошибка 123 123");
    // logger.warn("Предупреждение 123");
    // logger.success("все хорошо");


    yew::Renderer::<app>::new().render();
}
