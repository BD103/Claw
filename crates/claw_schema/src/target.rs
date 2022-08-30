use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::Block;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, (String, i64)>,
    // lists: HashMap<String, (String, Vec<i64>)>, // Change list type to enum?
    // broadcasts: HashMap<String, String>,
    pub blocks: HashMap<String, Block>,
    // comments: (),
    // current_costume: usize,
    pub costumes: Vec<()>,
    // sounds: Vec<()>,
    // volume: usize, // 0-100?
    // layer_order: usize,
    // tempo: usize,
    // video_transparency: usize,
    // video_state: String,
    // text_to_speech_language: (),
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
