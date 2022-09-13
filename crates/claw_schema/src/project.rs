use super::Target;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub targets: Vec<Target>,
    pub monitors: Vec<()>,
    pub extensions: Vec<()>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
