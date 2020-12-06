use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::errors::{Error};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use diesel::prelude::*;
use crate::DbConn;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    iss: String,
    exp: usize,
}

pub fn encode_jwt(id: String) -> String {
    let private_key = include_bytes!("keys/private.pem").to_vec();

    let my_claims = Claims{
            sub: id,
            iss: "onlysharks".to_string(),
            exp: (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + 2592000000) as usize
        };

    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(&*private_key)) {
        Ok(t) => t,
        Err(_) => panic!()
    };

    return token;
}

pub fn verify_jwt(jwt: String) -> Result<Claims, Error> {
    let private_key = include_bytes!("keys/private.pem").to_vec();

    let validation = Validation{
        leeway: 240,
        validate_exp: true,
        ..Validation::default()
    };

    let token_data = decode::<Claims>(&jwt, &DecodingKey::from_secret(&*private_key), &validation)?;

    Ok(token_data.claims)
}

fn is_valid(key: &str, request: &Request) -> bool {
    use crate::schema::users::dsl::*;

    let naked_key = key.replace("Bearer ", "");

    let key_vert = verify_jwt(naked_key.parse().unwrap()).unwrap();
    let mut valid_key = false;
    let database = request.guard::<DbConn>().unwrap();

    let results = users.filter(id.eq(key_vert.sub))
        .load::<crate::models::profile::Profile>(&*database)
        .expect("Error verifying key");

    for result in results {
        if result.authkey == naked_key {
            valid_key = true;
        }
    }

    return valid_key;
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for Claims {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0], request) => {
                let naked_key = keys[0].replace("Bearer ", "");
                let key = verify_jwt(naked_key.parse().unwrap()).unwrap();
                Outcome::Success(key)
            },
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}