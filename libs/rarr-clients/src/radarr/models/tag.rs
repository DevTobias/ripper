use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: u32,
    pub label: String,
}
