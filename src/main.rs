#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate requests;

mod db;
mod schema;
mod models;

use diesel::prelude::*;
use models::*;
use schema::device_list::dsl::*;
use dotenv::dotenv;
use std::env;

#[get("/")]
fn root() -> &'static str {
    "Hello world!"
}

#[get("/db")]
fn get_db() -> String {
    let mut ret = String::new();

    let conn = db::db_connect();
    let result = device_list.load::<Device>(&conn)
        .expect("Error loading devices");

    ret.push_str(&format!("Displaying {} things:\n\n", result.len()));
    for d in result {
        ret.push_str(&format!("- #{}: {} ({}), \"{}\"", d.id, d.name.unwrap_or("?".to_owned()), d.type_.unwrap_or("?".to_owned()), d.desc.unwrap_or("?".to_owned())));
    }

    ret
}

#[get("/feeder/on")]
fn feeder_on() -> &'static str {
    dotenv().ok();

    let feeder_ip = env::var("FEEDER_IP").expect("FEEDER_IP not set!");

    requests::get(&format!("http://{}/things/hatonif-feeder/properties/on", feeder_ip)).unwrap();

    "done"
}

fn main() {
    rocket::ignite().mount("/", routes![root, get_db]).launch();
}
