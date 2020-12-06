#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate jsonwebtoken;

use rocket_contrib::databases::{database};

pub mod endpoints;
pub mod models;
pub mod schema;
pub mod services;

#[database("db")]
pub struct DbConn(rocket_contrib::databases::diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        // Attach database
        .attach(DbConn::fairing())
        // Index
        .mount("/", routes![index])
        // Posts
        .mount("/api/posts", routes![endpoints::posts::new_post,
                                                endpoints::posts::posts_get,
                                                endpoints::posts::posts_delete,
                                                endpoints::posts::posts_patch])
        // Profiles
        .mount("/api/users", routes![endpoints::users::create_user,
                                                endpoints::users::login_user])
        // Search
        .launch();
}
