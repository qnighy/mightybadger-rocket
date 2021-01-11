use rocket::error::Error;
use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[get("/error")]
fn error() -> &'static str {
    panic!("/error is requested");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    mightybadger::setup();
    rocket::ignite()
        .mount("/", routes![index, ping, error])
        .attach(mightybadger_rocket::HoneybadgerHook::new())
        .launch()
        .await?;
    Ok(())
}
