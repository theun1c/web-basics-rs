mod logger;
use once_cell::sync::OnceCell;
use logger::Logger;
use yew::prelude::*;
use std::sync::Mutex;

static LOGGER: OnceCell<Mutex<Logger>> = OnceCell::new();

pub fn get_logger() -> &'static Mutex<Logger>{
    LOGGER.get_or_init(|| {
        let logger = Logger::new().expect("logger creating error");
        Mutex::new(logger)
    })
}

#[component]
fn app() -> Html {
    html! {
        <h1>{"Hello world"}</h1>
    }
}

fn main() { 
    // let mut logger = get_logger().lock().unwrap();
    // logger.error("Ошибка 123 123");
    // logger.warn("Предупреждение 123");
    // logger.success("все хорошо");


    yew::Renderer::<app>::new().render();
}
