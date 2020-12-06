use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, PartialEq, Insertable, QueryableByName)]
#[table_name="users"]
pub struct Profile {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub creationdate: chrono::NaiveDateTime,
    pub displayname: String,
    pub pronouns: String,
    pub description: String,
    pub birthday: chrono::NaiveDate,
    pub followers: i32,
    pub posts: Vec<String>,
    pub likedposts: Vec<String>,
    pub following: Vec<String>,
    pub authkey: String,
    pub pfp: String,
    pub banner: String
}

#[derive(Serialize, Deserialize, Queryable, PartialEq)]
pub struct PublicProfile {
    pub id: String,
    pub username: String,
    pub creationdate: chrono::NaiveDateTime,
    pub displayname: String,
    pub pronouns: String,
    pub description: String,
    pub birthday: chrono::NaiveDate,
    pub followers: i32,
    pub posts: Vec<String>,
    pub likedposts: Vec<String>,
    pub following: Vec<String>,
    pub pfp: String,
    pub banner: String
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateFormData {
    pub email: String,
    pub username: String,
    pub password: String,
    pub displayname: String,
    pub birthday: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct UserCreateData {
    pub email: String,
    pub username: String,
    pub password: String,
    pub displayname: String,
    pub birthday: chrono::NaiveDate,
    pub authkey: String
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct UserEditData {
    pub username: String,
    pub email: String,
    pub displayname: String,
    pub pronouns: String,
    pub description: String,
    pub birthday: chrono::NaiveDate,
    pub pfp: String,
    pub banner: String
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct UserEditPass {
    pub password: String,
}