use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub top_level: bool,
}
