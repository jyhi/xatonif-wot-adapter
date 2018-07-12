use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::HashMap;
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct DeviceNameIp {
    pub name: String,
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PropertyOn {
    on: bool
}

pub fn handler(dev_info: Arc<Mutex<HashMap<u32, DeviceNameIp>>>, ifttt: Arc<Mutex<HashMap<u32, u32>>>) {
    loop {
        thread::sleep(Duration::from_millis(500));
        for (this, that) in ifttt.lock().unwrap().iter() {
            let (if_dev_info, then_dev_info);
            {
                let lock_dev_info = dev_info.lock().unwrap();
                if_dev_info   = lock_dev_info.get(this).unwrap().clone();
                then_dev_info = lock_dev_info.get(that).unwrap().clone();
            }

            if hack_poll_thing_property_on(&if_dev_info.ip, &if_dev_info.name) == true {
                // XXX: Turn it off after 1s
                thread::Builder::new().name("hack_put".to_owned()).spawn(move || {
                    hack_put_thing_property_on(&then_dev_info.ip, &then_dev_info.name, true);
                    thread::sleep(Duration::from_secs(1));
                    hack_put_thing_property_on(&then_dev_info.ip, &then_dev_info.name, false);
                }).unwrap().join().unwrap();
            }
        }
    }
}

fn hack_poll_thing_property_on(ip: &str, name: &str) -> bool {
    let resp = Client::new()
        .get(&format!("http://{}/things/{}/properties/on", ip, name))
        .send();

    let json: PropertyOn;
    if let Some(mut resp) = resp.ok() {
        json = resp.json().unwrap();
        // eprintln!("GET http://{}/things/{}/properties/on -> {:?}", ip, name, json);
    } else {
        json = PropertyOn { on: false };
        eprintln!("GET http://{}/things/{}/properties/on -> ?", ip, name);
    }

    json.on
}

fn hack_put_thing_property_on(ip: &str, name: &str, v: bool) {
    let json = PropertyOn { on: v };

    eprintln!("PUT http://{}/things/{}/properties/on {:?} -> ?", ip, name, json);

    // let resp = Client::new()
    //     .put(&format!("http://{}/things/{}/properties/on", ip, name))
    //     .json(&PropertyOn { on: v })
    //     .send();

    // if let Some(mut resp) = resp.ok() {
    //     let json: PropertyOn = resp.json().unwrap();
    //     eprintln!("PUT http://{}/things/{}/properties/on -> {:?}", ip, name, json);
    // } else {
    //     eprintln!("PUT http://{}/things/{}/properties/on -> ?", ip, name);
    // }

    Client::new()
        .put(&format!("http://{}/things/{}/properties/on", ip, name))
        .json(&json)
        .send().unwrap();
}
