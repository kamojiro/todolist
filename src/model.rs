use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoEntry {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: u32,
    pub text: String,
}
