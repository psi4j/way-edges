use cosmic_text::{Color, FamilyOwned};
use educe::Educe;
use schemars::JsonSchema;
use serde::Deserialize;
use util::color::COLOR_BLACK;

use crate::widgets::common::{self, dt_family_owned, FamilyOwnedRef};

#[derive(Educe, Deserialize, JsonSchema, Clone)]
#[educe(Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
#[schemars(deny_unknown_fields)]
pub enum TextPreset {
    Time {
        #[serde(default = "dt_time_format")]
        format: String,
        #[serde(default)]
        time_zone: Option<String>,
        #[serde(default = "dt_time_update_interval")]
        update_interval: u64,
    },
    Custom {
        update_with_interval_ms: (u64, String),
    },
}
fn dt_time_format() -> String {
    "%Y-%m-%d %H:%M:%S".to_string()
}
fn dt_time_update_interval() -> u64 {
    1000
}

#[derive(Educe, Deserialize, JsonSchema, Clone)]
#[educe(Debug)]
#[schemars(deny_unknown_fields)]
pub struct TextConfig {
    #[serde(default = "dt_fg_color")]
    #[serde(deserialize_with = "common::color_translate")]
    #[schemars(schema_with = "common::schema_color")]
    pub fg_color: Color,
    #[serde(default = "dt_font_size")]
    pub font_size: i32,
    #[serde(default = "dt_family_owned")]
    #[serde(with = "FamilyOwnedRef")]
    pub font_family: FamilyOwned,

    pub preset: TextPreset,
}

fn dt_fg_color() -> Color {
    COLOR_BLACK
}
fn dt_font_size() -> i32 {
    24
}
