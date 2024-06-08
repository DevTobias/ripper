use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct QualityProfile {
    pub id: u32,
    pub name: String,
    pub language: QualityProfileLanguage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QualityProfileLanguage {
    pub id: u32,
    pub name: String,
}
