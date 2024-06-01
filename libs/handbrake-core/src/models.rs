use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Profile {
    pub label: String,
    pub name: String,
}
