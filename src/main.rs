extern crate curl;
extern crate diesel;
extern crate serde_json;

use std::io::*;
use curl::easy::Easy;

fn main() {
    let mut curl_handler = Easy::new();

    curl_handler.url("http://10.42.0.232/").unwrap();
    curl_handler.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    curl_handler.perform().unwrap();
}
