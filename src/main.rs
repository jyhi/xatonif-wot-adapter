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

use std::collections::HashMap;

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
fn get_hashmap(dev_ip: State<HashMap<u32, String>>, ifttt: State<HashMap<u32, u32>>) -> String {
    let mut ret = String::new();

    ret.push_str("Displaying std::collection::HashMap status:\n\n");
    ret.push_str(&format!("dev_ip = {:#?}", dev_ip.inner()));
    ret.push_str("\n");
    ret.push_str(&format!("ifttt = {:#?}", ifttt.inner()));

    ret
}

#[get("/<dev_id>/<prop>/<yn>")]
fn device_id_property_bool(dev_id: u32, prop: String, yn: bool, dev_ip: State<HashMap<u32, String>>) -> String {
    use schema::device_list::dsl::*;

    let mut ret = String::new();

    let ip = dev_ip.get(&dev_id).expect(&format!("Requested device ID {} does not exist!", dev_id));
    let dev_name = {
        let db_conn = db::db_connect();
        device_list.filter(id.eq(dev_id)).limit(1).load::<Device>(&db_conn).unwrap()[0].clone().name.unwrap()
    };
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", ip, dev_name, prop)).json(&json!({prop: yn})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n---\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

#[get("/<dev_id>/<prop>/<int>", rank = 1)]
fn device_id_property_uint(dev_id: u32, prop: String, int: u32, dev_ip: State<HashMap<u32, String>>) -> String {
    use schema::device_list::dsl::*;

    let mut ret = String::new();

    let ip = dev_ip.get(&dev_id).expect(&format!("Requested device ID {} does not exist!", dev_id));
    let dev_name = {
        let db_conn = db::db_connect();
        device_list.filter(id.eq(dev_id)).limit(1).load::<Device>(&db_conn).unwrap()[0].clone().name.unwrap()
    };
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", ip, dev_name, prop)).json(&json!({prop: int})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n---\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

#[get("/<dev_id>/<prop>/<string>", rank = 2)]
fn device_id_property_string(dev_id: u32, prop: String, string: u32, dev_ip: State<HashMap<u32, String>>) -> String {
    use schema::device_list::dsl::*;

    let mut ret = String::new();

    let ip = dev_ip.get(&dev_id).expect(&format!("Requested device ID {} does not exist!", dev_id));
    let dev_name = {
        let db_conn = db::db_connect();
        device_list.filter(id.eq(dev_id)).limit(1).load::<Device>(&db_conn).unwrap()[0].clone().name.unwrap()
    };
    let client = Client::new();
    let req = client.put(&format!("http://{}/things/{}/properties/{}", ip, dev_name, prop)).json(&json!({prop: string})).build().unwrap();

    ret.push_str(&format!("{:#?}", req));
    ret.push_str("\n---\n");
    ret.push_str(&format!("{:#?}", client.execute(req)));

    ret
}

#[get("/<device>/<prop>/<yn>", rank = 3)]
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

#[get("/<device>/<prop>/<int>", rank = 4)]
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

#[get("/<device>/<prop>/<string>", rank = 5)]
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

fn build_dev_ip_map() -> HashMap<u32, String> {
    use schema::device_list::dsl::*;

    let mut dev_ip: HashMap<u32, String> = HashMap::new();

    dotenv().ok();

    let devices = {
        let db_conn = db::db_connect();
        device_list.load::<Device>(&db_conn).expect("Error loading devices")
    };

    for d in devices {
        let env_name = d.name.unwrap_or("?".to_owned()).to_uppercase().replace("-", "_");
        if let Ok(ip) = env::var(&format!("{}_IP", env_name)) {
            dev_ip.insert(d.id, ip);
        }
    }

    dev_ip
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
    let dev_ip: HashMap<u32, String> = build_dev_ip_map();
    let ifttt: HashMap<u32, u32> = build_ifttt_map();

    rocket::ignite()
        .mount("/", routes![root,
                            get_db,
                            get_hashmap,
                            device_id_property_bool,
                            device_id_property_uint,
                            device_id_property_string,
                            device_property_bool,
                            device_property_uint,
                            device_property_string])
        .manage(dev_ip)
        .manage(ifttt)
        .launch();
}
