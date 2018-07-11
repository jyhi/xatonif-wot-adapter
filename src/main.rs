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
use schema::device_properties::dsl::*;
use schema::device_actions::dsl::*;
use schema::device_events::dsl::*;
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

    let (things, properties, actions, events);
    {
        let conn = db::db_connect();

        things = device_list.load::<Device>(&conn).expect("Error loading devices");
        properties = device_properties.load::<Property>(&conn).expect("Error loading properties");
        actions = device_actions.load::<Action>(&conn).expect("Error loading actions");
        events = device_events.load::<Event>(&conn).expect("Error loading events");
    }

    ret.push_str(&format!("There are total {} things recorded in the database:\n\n", things.len()));
    for t in things {
        ret.push_str(&format!("- #{}: {} ({}), \"{}\"\n", t.id, t.name.unwrap_or("?".to_owned()), t.type_.unwrap_or("?".to_owned()), t.desc.unwrap_or("?".to_owned())));
    }

    ret.push_str("\n");

    ret.push_str(&format!("There are total {} properties recorded in the database:\n\n", properties.len()));
    for p in properties {
        ret.push_str(&format!("- #{}: {} ({}), \"{}\", at {}\n", p.id, p.name.unwrap_or("?".to_owned()), p.type_.unwrap_or("?".to_owned()), p.desc.unwrap_or("?".to_owned()), p.href.unwrap_or("?".to_owned())));
    }

    ret.push_str("\n");

    ret.push_str(&format!("There are total {} actions recorded in the database:\n\n", actions.len()));
    for a in actions {
        ret.push_str(&format!("- #{}: {}, \"{}\"\n", a.id, a.name.unwrap_or("?".to_owned()), a.desc.unwrap_or("?".to_owned())));
    }

    ret.push_str("\n");

    ret.push_str(&format!("There are total {} events recorded in the database:\n\n", events.len()));
    for e in events {
        ret.push_str(&format!("- #{}: {}, \"{}\"\n", e.id, e.name.unwrap_or("?".to_owned()), e.desc.unwrap_or("?".to_owned())));
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

#[get("/<device>/<prop>/<string>", rank = 2)]
fn device_property_string(device: String, prop: String, string: String) -> String {
    let mut ret = String::new();

    dotenv().ok();

    let env_name = device.replace("-", "_").to_uppercase();
    let ip = env::var(&format!("{}_IP", env_name)).expect(&format!("{}_IP not set!", env_name));
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", ip, device, prop)).json(&json!({prop: string})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n---\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

fn main() {
    rocket::ignite().mount("/", routes![root, get_db, device_property_bool, device_property_uint, device_property_string]).launch();
}
