use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PostPage {
    pub posts: Vec<Post>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
}
