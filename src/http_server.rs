#[get("/")]
fn root() -> &'static str {
    "Hello world!"
}
