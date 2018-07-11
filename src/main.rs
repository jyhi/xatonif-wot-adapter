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
mod device;
mod schema;
mod models;
mod handler;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket::State;

use diesel::prelude::*;
use models::*;

use dotenv::dotenv;
use std::env;

use reqwest::Client;

#[get("/")]
fn root() -> &'static str {
    "Hello world!"
}

#[get("/db")]
fn get_db() -> String {
    use schema::device_list::dsl::*;
    use schema::device_properties::dsl::*;
    use schema::device_actions::dsl::*;
    use schema::device_events::dsl::*;
    use schema::task_edition::dsl::*;

    let mut ret = String::new();

    let (things, properties, actions, events, editions);
    {
        let conn = db::db_connect();

        things = device_list.load::<Device>(&conn).expect("Error loading devices");
        properties = device_properties.load::<Property>(&conn).expect("Error loading properties");
        actions = device_actions.load::<Action>(&conn).expect("Error loading actions");
        events = device_events.load::<Event>(&conn).expect("Error loading events");
        editions = task_edition.load::<Ifttt>(&conn).expect("Error loading editions");
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

    ret.push_str("\n");

    ret.push_str(&format!("There are total {} editions recorded in the database:\n\n", editions.len()));
    for e in editions {
        ret.push_str(&format!("- #{}: {} -> {}\n", e.id, e.if_dev_id, e.then_dev_id));
    }

    ret
}

#[get("/hashmap")]
fn get_hashmap(dev_info: State<Mutex<HashMap<u32, DeviceNameIp>>>, ifttt: State<Mutex<HashMap<u32, u32>>>) -> String {
    let mut ret = String::new();

    let locked_dev_info = dev_info.lock().unwrap();
    let locked_ifttt    = ifttt.lock().unwrap();

    ret.push_str("Displaying std::collection::HashMap status:\n\n");
    ret.push_str(&format!("dev_info = {:#?}", *locked_dev_info));
    ret.push_str("\n");
    ret.push_str(&format!("ifttt = {:#?}", *locked_ifttt));

    ret
}

#[get("/reload")]
fn reload_ifttt(ifttt: State<Mutex<HashMap<u32, u32>>>) -> &'static str {
    let ifttt_from_db = build_ifttt_map();

    let mut locked_ifttt = ifttt.lock().unwrap();

    locked_ifttt.clear();
    for rel in ifttt_from_db {
        locked_ifttt.insert(rel.0, rel.1);
    }

    "done"
}

#[get("/<dev_id>/<prop>/<yn>", rank = 0)]
fn device_property_bool(dev_id: u32, prop: String, yn: bool, dev_info: State<Mutex<HashMap<u32, DeviceNameIp>>>) -> String {
    let mut ret = String::new();

    let lock = dev_info.lock().unwrap();
    let dev = lock.get(&dev_id).unwrap();
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", dev.ip, dev.name, prop)).json(&json!({prop: yn})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n\n---\n\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

#[get("/<dev_id>/<prop>/<int>", rank = 1)]
fn device_property_uint(dev_id: u32, prop: String, int: u32, dev_info: State<Mutex<HashMap<u32, DeviceNameIp>>>) -> String {
    let mut ret = String::new();

    let lock = dev_info.lock().unwrap();
    let dev = lock.get(&dev_id).unwrap();
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", dev.ip, dev.name, prop)).json(&json!({prop: int})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n\n---\n\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

#[get("/<dev_id>/<prop>/<string>", rank = 2)]
fn device_property_string(dev_id: u32, prop: String, string: String, dev_info: State<Mutex<HashMap<u32, DeviceNameIp>>>) -> String {
    let mut ret = String::new();

    let lock = dev_info.lock().unwrap();
    let dev = lock.get(&dev_id).unwrap();
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", dev.ip, dev.name, prop)).json(&json!({prop: string})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n---\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

fn build_dev_info_map() -> HashMap<u32, DeviceNameIp> {
    use schema::device_list::dsl::*;

    let mut dev_info: HashMap<u32, DeviceNameIp> = HashMap::new();

    dotenv().ok();

    let devices = {
        let db_conn = db::db_connect();
        device_list.load::<Device>(&db_conn).expect("Error loading devices")
    };

    for d in devices {
        let dev_name = d.name.unwrap_or("?".to_owned());
        let env_name = dev_name.to_uppercase().replace("-", "_");
        if let Ok(dev_ip) = env::var(&format!("{}_IP", env_name)) {
            dev_info.insert(d.id, DeviceNameIp { name: dev_name, ip: dev_ip });
        }
    }

    dev_info
}

fn build_ifttt_map() -> HashMap<u32, u32> {
    use schema::task_edition::dsl::*;

    let mut ifttt: HashMap<u32, u32> = HashMap::new();

    let rels = {
        let db_conn = db::db_connect();
        task_edition.load::<Ifttt>(&db_conn).expect("Error loading task IFTTTs")
    };

    for r in rels {
        ifttt.insert(r.if_dev_id, r.then_dev_id);
    }

    ifttt
}

fn main() {
    let dev_info: HashMap<u32, DeviceNameIp> = build_dev_info_map();
    let ifttt: HashMap<u32, u32> = build_ifttt_map();

    handler::handler(Mutex::new(dev_info), Mutex::new(ifttt));

    rocket::ignite()
        .mount("/", routes![root,
                            reload_ifttt,
                            get_db,
                            get_hashmap,
                            device_property_bool,
                            device_property_uint,
                            device_property_string])
        .manage(Mutex::new(dev_info))
        .manage(Mutex::new(ifttt))
        .launch();
}
