use super::Target;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub targets: Vec<Target>,
    pub monitors: Vec<()>,
    pub extensions: Vec<()>,
    pub meta: Meta,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            targets: Vec::new(),
            monitors: Vec::new(),
            extensions: Vec::new(),
            meta: Meta::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub semver: String,
    pub vm: String,
    pub agent: String,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            semver: "3.0.0".to_string(),
            vm: "1.1.2".to_string(),
            agent: "Claw by BD103 <https://bd103.github.io>".to_string(),
        }
    }
}
