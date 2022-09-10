use serde::{
    Deserialize,
    Serialize,
};

use crate::keyboard::VirtualKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Padding {
    pub monitor: u64,
    pub workspace: u64,
    pub padding: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirectionHotKey {
    pub left: Option<VirtualKey>,
    pub right: Option<VirtualKey>,
    pub up: Option<VirtualKey>,
    pub down: Option<VirtualKey>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AxisHotKey {
    pub horizontal: Option<VirtualKey>,
    pub vertical: Option<VirtualKey>,
    pub horizontal_vertical: Option<VirtualKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeyBindings {
    pub quit: Option<VirtualKey>,
    pub move_window: Option<DirectionHotKey>,
    pub axis_increase: Option<AxisHotKey>,
    pub axis_decrease: Option<AxisHotKey>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Category {
    Float,
    Manage,
    NameChange,
    Overflow,
    Tray,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Rule {
    Class { name: String },
    Exe { name: String },
    Title { name: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Window {
    pub categories: Vec<Category>,
    pub rule: Vec<Rule>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub container_padding: Option<Padding>,
    pub workspace_padding: Option<Padding>,
    pub key_bindings: Option<KeyBindings>,
    pub window: Option<Vec<Window>>,
}
