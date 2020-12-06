use chrono::prelude::*;
use diesel::prelude::*;
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::{DbConn, models};
use crate::models::post::NewPost;
use regex::Regex;

#[get("/<postid>")]
pub fn posts_get(postid: String, conn: DbConn) -> Json<models::post::Post> {
    use crate::schema::posts::dsl::*;

    let results = posts.filter(id.eq(postid))
        .load::<crate::models::post::Post>(&*conn)
        .expect("Error loading post");

    let mut post_data = models::post::Post{
        id: "404".parse().unwrap(),
        creatorid: "404".parse().unwrap(),
        date: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        content: "404".parse().unwrap(),
        images: None,
        likes: 0
    };

    for result in results {
        post_data = models::post::Post{
            id: result.id,
            creatorid: result.creatorid,
            date: result.date,
            content: result.content,
            images: result.images,
            likes: result.likes
        }
    }

    return Json(post_data);
}

#[delete("/<postid>")]
pub fn posts_delete(postid: String, conn: DbConn, key: crate::services::jwt::Claims) -> Status {
    use crate::schema::posts::dsl::*;

    let mut correct_user = false;

    let results = posts.filter(id.eq(&postid))
        .load::<crate::models::post::Post>(&*conn)
        .expect("Error verifying key");

    for result in results {
        if result.creatorid == key.sub {
            correct_user = true;
        }
    }

    if !correct_user {
        return Status::Unauthorized;
    }

    diesel::delete(posts.filter(id.eq(postid)))
        .execute(&*conn)
        .expect("Error deleting post");

    return Status::Ok;
}

#[patch("/<postid>", data = "<data>")]
pub fn posts_patch(data: Json<models::post::EditPost>, postid: String, conn: DbConn, key: crate::services::jwt::Claims) -> Status {
    use crate::schema::posts::dsl::*;

    let mut correct_user = false;

    let results = posts.filter(id.eq(&postid))
        .load::<crate::models::post::Post>(&*conn)
        .expect("Error verifying key");

    for result in results {
        if result.creatorid == key.sub {
            correct_user = true;
        }
    }

    if !correct_user {
        return Status::Unauthorized;
    }

    if Regex::new("[^i*🦈]").unwrap().is_match(&*data.0.content) || &*data.0.content == "" {
        return Status::BadRequest;
    }

    diesel::update(posts.find(&postid))
        .set(content.eq(data.0.content))
        .get_result::<crate::models::post::Post>(&*conn)
        .expect("Error updating post");

    return Status::Ok;
}

#[post("/new", data = "<data>")]
pub fn new_post(data: Json<models::post::NewPostReq>, conn: DbConn, key: crate::services::jwt::Claims) -> Status {
    use crate::schema::posts::dsl::*;

    if Regex::new("[^i*🦈]").unwrap().is_match(&*data.0.content) || &*data.0.content == "" {
        return Status::BadRequest;
    }

    let new_post = NewPost{
        creatorid: key.sub.clone(),
        content: data.0.content,
        images: data.0.images
    };

    let post = diesel::insert_into(posts)
        .values(&new_post)
        .get_result::<crate::models::post::Post>(&*conn)
        .expect("Error saving new post");


    diesel::sql_query(format!("UPDATE users SET posts = array_append(posts, '{}') WHERE id='{}';", post.id, &key.sub))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error updating user's post list");

    return Status::Ok;
}