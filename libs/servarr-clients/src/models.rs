use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rootfolder {
    pub id: u32,
    #[serde(alias = "freeSpace")]
    pub free_space: u64,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QualityProfile {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: u32,
    pub label: String,
}
