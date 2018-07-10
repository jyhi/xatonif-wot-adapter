use diesel::mysql::types::Datetime;

#[derive(Queryable)]
pub struct Device {
    id: usize,
    name: Option<String>,
    type_: Option<String>,
    desc: Option<String>,
    apid: usize,
    aaid: usize,
    aeid: usize,
}

#[derive(Queryable)]
pub struct Property {
    id: usize,
    name: Option<String>,
    type_: Option<String>,
    desc: Option<String>,
    href: Option<String>,
}

#[derive(Queryable)]
pub struct Action {
    id: usize,
    name: Option<String>,
    desc: Option<String>,
}

#[derive(Queryable)]
pub struct Event {
    id: usize,
    name: Option<String>,
    desc: Option<String>,
}
