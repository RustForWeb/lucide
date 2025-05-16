use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Metadata {
    pub categories: Vec<String>,
}
