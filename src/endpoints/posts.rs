use rocket_contrib::json::{Json, JsonValue};
use crate::models;

#[get("/<postid>")]
fn posts_get(postid: String) -> &'static str {
    "Hello, world!"
}

#[delete("/<postid>")]
fn posts_delete(postid: String) -> &'static str {
    "Hello, world!"
}

#[patch("/<postid>")]
fn posts_patch(postid: String) -> &'static str {
    "Hello, world!"
}

#[post("/new", data = "<data>")]
pub fn new_post(data: Json<models::post::NewPost>) -> &'static str {
    println!("{}", data.0.content);
    println!("Images? {}", data.0.images.is_some());
    for x in data.0.images.unwrap() {
        println!("{}", x);
    }
    return "t";
}