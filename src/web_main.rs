// ru: данная строчка говорит о том, 
// что нужно использовать макросы из указанного крейта
// в данном случае используется макрос router!
#[macro_use] extern crate nickel;

use nickel::Nickel;

fn hello() -> &'static str {
    "Hello world!"
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            hello()
        }
    });

    server.listen("127.0.0.1:6767");
}

