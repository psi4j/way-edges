use schemars::JsonSchema;
use serde::Deserialize;

use super::Config;

pub type GroupConfig = Vec<Config>;

#[derive(Deserialize, Debug, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct Group {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub widgets: Vec<Config>,
}
#[derive(Deserialize, Debug, JsonSchema)]
#[schemars(extend("allowTrailingCommas" = true))]
pub struct Root {
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(default)]
    pub ensure_load_group: Vec<String>,
}

impl Root {
    pub fn take_group(&mut self, name: &str) -> Option<Group> {
        let position = self.groups.iter().position(|g| g.name == name)?;
        Some(self.groups.swap_remove(position))
    }
    pub fn take_first(&mut self) -> Option<Group> {
        if !self.groups.is_empty() {
            Some(self.groups.swap_remove(0))
        } else {
            None
        }
    }
}

pub fn parse_config(data: &str) -> Result<Root, String> {
    serde_jsonrc::from_str(data).map_err(|e| format!("JSON parse error: {e}"))
}
