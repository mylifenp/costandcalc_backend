use async_graphql::{ID, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub id: ID,
    pub url: String,
}
