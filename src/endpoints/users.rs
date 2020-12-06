use diesel::prelude::*;
use regex::Regex;
use rocket::http::Status;
use rocket::request::Form;
use rocket_contrib::json::Json;
use sodiumoxide::crypto::pwhash::argon2id13;

use crate::{DbConn, models};
use crate::models::profile::UserCreateData;

#[post("/create", data = "<data>")]
pub fn create_user(data: Form<models::profile::UserCreateFormData>, conn: DbConn) -> String {
    use crate::schema::users::dsl::*;
    sodiumoxide::init().unwrap();

    let hash = argon2id13::pwhash(
        data.0.password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE
    ).unwrap();

    let passhash = std::str::from_utf8(&hash.0).unwrap().to_string();

    let new_account = UserCreateData{
        email: data.0.email,
        username: data.0.username,
        password: passhash.trim_end_matches('\u{0}').parse().unwrap(),
        displayname: data.0.displayname,
        birthday: chrono::NaiveDate::parse_from_str(&*data.0.birthday, "%Y-%m-%d").unwrap()
    };

    let result = diesel::insert_into(users)
        .values(&new_account)
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Error creating new user");

    let new_user_id = result.id;
    let new_jwt = crate::services::jwt::encode_jwt((&*new_user_id).parse().unwrap());

    diesel::update(users.find(new_user_id))
        .set(authkey.eq(&new_jwt))
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Error creating new JWT");

    return new_jwt;
}

#[post("/login", data = "<data>")]
pub fn login_user(data: Json<models::post::EditPost>, conn: DbConn) -> Status {
    use crate::schema::posts::dsl::*;
    sodiumoxide::init().unwrap();

    let user =

    return Status::Ok;
}