use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    pub name: String,
    pub data_format: String,
    pub asset_id: String,
    pub md5ext: String,
}
