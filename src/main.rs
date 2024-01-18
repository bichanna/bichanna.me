mod entities;

use rocket::{self, get, launch, routes, Build, Rocket};

#[get("/ping")]
fn ping() -> &'static str {
    "Hello, world. The site is running."
}

#[launch]
fn launch() -> Rocket<Build> {
    rocket::build().mount("/", routes![ping])
}
