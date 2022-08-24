use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    is_stage: bool,
    name: String,
    variables: HashMap<String, (String, i64)>,
    lists: HashMap<String, (String, Vec<i64>)>, // Change list type to enum?
    broadcasts: HashMap<String, String>,
    blocks: HashMap<String, ()>,
    #[serde(skip)]
    comments: (),
    current_costume: usize,
    costumes: Vec<()>,
    sounds: Vec<()>,
    volume: usize, // 0-100?
    layer_order: usize,
    tempo: usize,
    video_transparency: usize,
    video_state: String,
    text_to_speech_language: (),
}

impl Target {
    pub fn new_sprite(name: String) -> Self {
        Target {
            is_stage: false,
            name,
            variables: HashMap::new(),
            lists: HashMap::new(),
            broadcasts: HashMap::new(),
            blocks: HashMap::new(),
            comments: (),
            current_costume: 0,
            costumes: Vec::new(),
            sounds: Vec::new(),
            volume: 100,
            layer_order: 0,
            tempo: 60,
            video_transparency: 50,
            video_state: "on".to_string(),
            text_to_speech_language: (),
        }
    }

    pub fn new_stage() -> Self {
        Target {
            is_stage: true,
            ..Self::new_sprite("Stage".to_string())
        }
    }

    pub fn insert_variable(&mut self, name: String, default: i64) {
        self.variables.insert(name.clone(), (name, default));
    }

    pub fn insert_list(&mut self, name: String, default: Vec<i64>) {
        self.lists.insert(name.clone(), (name, default));
    }

    pub fn insert_broadcast(&mut self, name: String) {
        self.broadcasts.insert(name.clone(), name);
    }
}
