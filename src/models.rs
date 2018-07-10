#[derive(Queryable)]
pub struct Device {
    pub id: u32,
    pub name: Option<String>,
    pub type_: Option<String>,
    pub desc: Option<String>,
    pub apid: u32,
    pub aaid: u32,
    pub aeid: u32,
}

#[derive(Queryable)]
pub struct Property {
    pub id: u32,
    pub name: Option<String>,
    pub type_: Option<String>,
    pub desc: Option<String>,
    pub href: Option<String>,
}

#[derive(Queryable)]
pub struct Action {
    pub id: u32,
    pub name: Option<String>,
    pub desc: Option<String>,
}

#[derive(Queryable)]
pub struct Event {
    pub id: u32,
    pub name: Option<String>,
    pub desc: Option<String>,
}
