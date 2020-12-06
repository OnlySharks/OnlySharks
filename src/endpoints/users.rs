use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;
use sodiumoxide::crypto::pwhash::argon2id13;

use crate::{DbConn, models};
use crate::models::profile::UserCreateData;

#[post("/create", data = "<data>")]
pub fn create_user(data: Json<models::profile::UserCreateFormData>, conn: DbConn) -> String {
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
        birthday: chrono::NaiveDate::parse_from_str(&*data.0.birthday, "%Y-%m-%d").unwrap(),
        authkey: "".to_string()
    };

    let result = diesel::insert_into(users)
        .values(&new_account)
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Error creating new user");

    let new_user_id = result.id;
    let new_jwt = crate::services::jwt::encode_jwt((&*new_user_id).parse().unwrap());

    diesel::update(users.find(&new_user_id))
        .set(authkey.eq(&new_jwt))
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Error creating new JWT");

    return format!("{},{}", new_jwt, new_user_id);
}

#[post("/login", data = "<data>")]
pub fn login_user(data: Json<models::profile::LoginData>, conn: DbConn) -> String {
    use crate::schema::users::dsl::*;
    sodiumoxide::init().unwrap();

    let mut hashed_pass_from_db: String = "".to_string();
    let mut hash_padding = [0u8; 128];
    let mut user_id = "".to_string();

    let results = users.filter(username.eq(data.0.username))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error loading user");

    for result in results {
        hashed_pass_from_db = result.password;
        user_id = result.id;
    }

    hashed_pass_from_db
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, byte)|{
            hash_padding[i] = byte.clone();
        });

    let pass_check = match argon2id13::HashedPassword::from_slice(&hash_padding) {
        Some(hp) => argon2id13::pwhash_verify(&hp, data.0.password.as_bytes()),
        _ => false,
    };

    return if pass_check {
        let new_jwt = crate::services::jwt::encode_jwt((&*user_id).parse().unwrap());

        diesel::update(users.find(&user_id))
            .set(authkey.eq(&new_jwt))
            .get_result::<crate::models::profile::Profile>(&*conn)
            .expect("Error creating new JWT");

        format!("{},{}", new_jwt, user_id)
    } else {
        "err".to_string()
    }
}

#[post("/logout")]
pub fn logout_user(conn: DbConn, key: crate::services::jwt::Claims) -> Status{
    use crate::schema::users::dsl::*;

    diesel::update(users.find(&key.sub))
        .set(authkey.eq("".to_string()))
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Error revoking new JWT");

    return Status::Ok;
}

#[get("/<userid>")]
pub fn show_user(conn: DbConn, userid: String) -> Json<crate::models::profile::PublicProfile> {
    use crate::schema::users::dsl::*;

    let results = users.filter(id.eq(userid))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error loading profile");

    let mut userprofile = crate::models::profile::PublicProfile{
        id: "404".to_string(),
        username: "404".to_string(),
        creationdate: chrono::NaiveDate::from_ymd(2020, 12, 4).and_hms(10, 00, 00),
        displayname: "404".to_string(),
        pronouns: "404".to_string(),
        description: "404".to_string(),
        birthday: chrono::NaiveDate::from_ymd(2020, 12, 4),
        followers: 404,
        posts: vec![],
        likedposts: vec![],
        following: vec![],
        pfp: "404".to_string(),
        banner: "404".to_string()
    };

    for profile in results {
        userprofile = crate::models::profile::PublicProfile{
            id: profile.id,
            username: profile.username,
            creationdate: profile.creationdate,
            displayname: profile.displayname,
            pronouns: profile.pronouns,
            description: profile.description,
            birthday: profile.birthday,
            followers: profile.followers,
            posts: profile.posts,
            likedposts: profile.likedposts,
            following: profile.following,
            pfp: profile.pfp,
            banner: profile.banner
        };
    }

    return Json(userprofile);
}

#[get("/getid/<reqname>")]
pub fn get_user_id(conn: DbConn, reqname: String) -> String {
    use crate::schema::users::dsl::*;

    let results = users.filter(username.eq(reqname))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error loading profile");

    let mut userid = "404".to_string();

    for profile in results {
        userid = profile.id;
    }

    return userid;
}

#[post("/edit", data = "<data>")]
pub fn edit_user(data: Json<models::profile::UserEditData>, conn: DbConn, key: crate::services::jwt::Claims) -> Status{
    use crate::schema::users::dsl::*;

    diesel:: update(users.find(&key.sub))
        .set((
            username.eq(data.0.username),
            email.eq(data.0.email),
            displayname.eq(data.0.displayname),
            pronouns.eq(data.0.pronouns),
            description.eq(data.0.description),
            birthday.eq(data.0.birthday),
            pfp.eq(data.0.pfp),
            banner.eq(data.0.banner)
        ))
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Unable to update user profile");

    return Status::Ok;
}

#[post("/editpass", data = "<data>")]
pub fn edit_user_pass(data: Json<models::profile::UserEditPass>, conn: DbConn, key: crate::services::jwt::Claims) -> String {
    use crate::schema::users::dsl::*;
    sodiumoxide::init().unwrap();

    let hash = argon2id13::pwhash(
        data.0.password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE
    ).unwrap();

    let passhash: String = std::str::from_utf8(&hash.0).unwrap().to_string()
        .trim_end_matches('\u{0}').parse().unwrap();

    diesel:: update(users.find(&key.sub))
        .set(password.eq(passhash))
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Unable to update user profile");

    let new_jwt = crate::services::jwt::encode_jwt((&key.sub).parse().unwrap());

    diesel::update(users.find(&key.sub))
        .set(authkey.eq(&new_jwt))
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Error creating new JWT");

    return new_jwt;
}

#[put("/<userid>/follow")]
pub fn follow_user(userid: String, conn: DbConn, key: crate::services::jwt::Claims) -> String {
    use crate::schema::users::dsl::*;

    let mut already_followed = false;

    let profile_results = users.filter(id.eq(&key.sub))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error getting user's data");

    for result in profile_results {
        if result.following.contains(&userid) {
            already_followed = true;
        }
    }

    if already_followed {
        return "Already following".to_string();
    }

    diesel::update(users.find(&userid))
        .set(followers.eq(followers + 1))
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Error following user");

    diesel::sql_query(format!("UPDATE users SET following = array_append(following, '{}') WHERE id='{}';", userid, &key.sub))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error updating user's following list");

    return "Now following".to_string();
}

#[delete("/<userid>/follow")]
pub fn unfollow_user(userid: String, conn: DbConn, key: crate::services::jwt::Claims) -> String {
    use crate::schema::users::dsl::*;

    let mut already_followed = false;

    let profile_results = users.filter(id.eq(&key.sub))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error getting user's data");

    for result in profile_results {
        if result.following.contains(&userid) {
            already_followed = true;
        }
    }

    if !already_followed {
        return "Not following".to_string();
    }

    diesel::update(users.find(&userid))
        .set(followers.eq(followers - 1))
        .get_result::<crate::models::profile::Profile>(&*conn)
        .expect("Error unfollowing user");

    diesel::sql_query(format!("UPDATE users SET following = array_remove(following, '{}') WHERE id='{}';", userid, &key.sub))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error updating user's following list");

    return "Unfollowed".to_string();
}