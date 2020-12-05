use crate::schema::posts;

#[derive(Serialize, Deserialize, Queryable, PartialEq)]
pub struct Post {
    pub id: String,
    pub creatorid: String,
    pub date: chrono::NaiveDateTime,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub likes: i32
}

#[derive(Deserialize)]
pub struct EditPost {
    pub content: String,
}

#[derive(Deserialize)]
pub struct NewPostReq {
    pub content: String,
    pub images: Option<Vec<String>>,
}

#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub creatorid: String,
    pub content: String,
    pub images: Option<Vec<String>>,
}