extern crate curl;

use curl::easy::Easy;

fn http_get(url: &str) -> String {
    let mut response = Vec::new();
    let mut handle   = Easy::new();

    handle.url(url).unwrap();

    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            response.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();

        transfer.perform().unwrap();
    }

    String::from_utf8(response).unwrap()
}

fn http_put(url: &str, data: &str) {
    let mut payload = data.as_bytes();
    let mut handle = Easy::new();

    let mut headers = List::new();
    headers.append("Content-Type: application/json").unwrap();
    headers.append("Expect:").unwrap();

    handle.url(url).unwrap();

    handle.upload(true).unwrap();
    handle.http_headers(headers).unwrap();
    handle.post_field_size(data.len() as u64).unwrap();

    let mut transfer = handle.transfer();
    transfer.read_function(|buf| {
        Ok(payload.read(buf).unwrap_or(0))
    }).unwrap();

    transfer.perform().unwrap();
}
