#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod endpoints;
pub mod models;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        // Index
        .mount("/", routes![index])
        // Posts
        .mount("/api/posts", routes![endpoints::posts::new_post])
        // Profiles
        // Search
        .launch();
}
