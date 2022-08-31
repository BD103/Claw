use crate::Block;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, (String, i64)>,
    pub blocks: HashMap<String, Block>,
    pub costumes: Vec<()>,
}

impl Target {
    pub fn new_sprite(name: String) -> Self {
        Target {
            is_stage: false,
            name,
            variables: HashMap::new(),
            blocks: HashMap::new(),
            costumes: Vec::new(),
        }
    }

    pub fn new_stage() -> Self {
        Target {
            is_stage: true,
            ..Self::new_sprite("Stage".to_string())
        }
    }
}
