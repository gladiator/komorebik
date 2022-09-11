use std::collections::HashMap;

use komorebi_core::{
    ApplicationIdentifier,
    SocketMessage,
};
use serde::Deserialize;

use crate::keyboard::VirtualKey;

#[derive(Clone, Debug, Deserialize)]
pub struct Rule {
    #[serde(rename = "type")]
    pub identifier: ApplicationIdentifier,
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Window {
    pub bordered: Option<bool>,
    pub floating: Option<bool>,
    pub layered: Option<bool>,
    pub managed: Option<bool>,
    pub object_name_change: Option<bool>,
    pub tray: Option<bool>,
    #[serde(rename = "rule")]
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Konfig {
    pub default: Vec<SocketMessage>,
    pub keys: HashMap<VirtualKey, SocketMessage>,
    #[serde(rename = "window")]
    pub windows: Vec<Window>,
}
