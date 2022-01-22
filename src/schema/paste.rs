use serde::Deserialize;
use crate::{Uuid, DateTime, Utc};

#[derive(Deserialize)]
pub struct Paste {
    pub id: Uuid,
    pub creator: String,
    pub creation: DateTime<Utc>,
    pub content: String,
    pub unlisted: bool,
    pub title: Option<String>,
    pub description: Option<String>,
}
