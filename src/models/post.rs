#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub date: String,
    pub content: String,
    pub images: Vec<String>,
    pub likes: i32
}

#[derive(Deserialize)]
pub struct NewPost {
    pub content: String,
    pub images: Option<Vec<String>>,
}