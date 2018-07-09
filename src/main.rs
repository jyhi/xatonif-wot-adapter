#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

// extern crate serde;
// // #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;

mod http_server;

fn main() {
    rocket::ignite().mount("/", routes![http_server::root]).launch();
}
