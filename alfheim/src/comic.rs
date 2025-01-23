use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Comic {
    id: String,
    title: String,
    author: String,
    condition: String,
}