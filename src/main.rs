#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate serde;
#[macro_use]
extern crate serde_json;
// extern crate serde_derive;

extern crate reqwest;

mod db;
mod schema;
mod models;

use diesel::prelude::*;
use models::*;
use schema::device_list::dsl::*;
use dotenv::dotenv;
use std::env;
use reqwest::Client;

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

#[get("/<device>/<prop>/<yn>")]
fn device_property_bool(device: String, prop: String, yn: bool) -> String {
    let mut ret = String::new();

    dotenv().ok();

    let env_name = device.replace("-", "_").to_uppercase();
    let ip = env::var(&format!("{}_IP", env_name)).expect(&format!("{}_IP not set!", env_name));
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", ip, device, prop)).json(&json!({prop: yn})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n---\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

#[get("/<device>/<prop>/<int>", rank = 1)]
fn device_property_uint(device: String, prop: String, int: u32) -> String {
    let mut ret = String::new();

    dotenv().ok();

    let env_name = device.replace("-", "_").to_uppercase();
    let ip = env::var(&format!("{}_IP", env_name)).expect(&format!("{}_IP not set!", env_name));
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", ip, device, prop)).json(&json!({prop: int})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n---\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

fn main() {
    rocket::ignite().mount("/", routes![root, get_db, device_property_bool, device_property_uint]).launch();
}
