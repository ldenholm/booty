use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comic {
    id: String,
    title: String,
    author: String,
    condition: String,
}