use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rootfolder {
    pub id: u32,
    #[serde(alias = "freeSpace")]
    pub free_space: u64,
    pub path: String,
}
