#[derive(Serialize, Deserialize)]
pub struct UserSearchResults {
    pub(crate) results: Vec<String>
}