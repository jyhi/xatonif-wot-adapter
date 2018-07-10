use self::db;

#[get("/")]
fn root() -> &'static str {
    "Hello world!"
}

#[get("/db")]
fn get_db() -> String {
    let conn = db::db_connect();

    "Test".to_owned()
}
