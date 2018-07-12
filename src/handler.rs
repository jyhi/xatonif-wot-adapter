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

#[derive(Serialize, Deserialize)]
struct JsonSingleKeyVal {
    key: String,
    val: bool,
}

pub fn handler(dev_info: Arc<Mutex<HashMap<u32, DeviceNameIp>>>, ifttt: Arc<Mutex<HashMap<u32, u32>>>) {
    for (this, that) in ifttt.lock().unwrap().iter() {
        let (if_dev_info, then_dev_info);
        {
            let lock_dev_info = dev_info.lock().unwrap();
            if_dev_info   = lock_dev_info.get(this).unwrap().clone();
            then_dev_info = lock_dev_info.get(that).unwrap().clone();
        }

        if hack_poll_thing_property_on(&if_dev_info.ip, &if_dev_info.name) == true {
            // XXX: Turn it off after 1s
            thread::spawn(move || {
                hack_put_thing_property_on(&then_dev_info.ip, &then_dev_info.name, true);
                thread::sleep(Duration::from_secs(1));
                hack_put_thing_property_on(&then_dev_info.ip, &then_dev_info.name, false);
            }).join().unwrap();
        }
    }
}

fn hack_poll_thing_property_on(ip: &str, name: &str) -> bool {
    println!("http://{}/things/{}/properties/on", ip, name);

    let resp: JsonSingleKeyVal = Client::new()
        .get(&format!("http://{}/things/{}/properties/on", ip, name))
        .send().unwrap()
        .json().unwrap();

    resp.val
}

fn hack_put_thing_property_on(ip: &str, name: &str, v: bool) -> bool {
    let resp: JsonSingleKeyVal = Client::new()
        .put(&format!("http://{}/things/{}/properties/on", ip, name))
        .json(&JsonSingleKeyVal { key: "on".to_owned(), val: v})
        .send().unwrap()
        .json().unwrap();

    resp.val
}
