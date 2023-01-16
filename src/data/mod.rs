pub mod pg;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Blob {
    id: i64,
    data: serde_json::Value
}

