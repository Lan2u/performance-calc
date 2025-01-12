#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use tracing::{info, Level};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let pub_dir = "./public";
    info!("Using public directory {:?}", pub_dir);

    rocket::build()
        .mount("/api", routes![index])
        .mount("/", FileServer::from(pub_dir))
}
