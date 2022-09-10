use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    keyboard::VirtualKey,
    message::Message,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub struct Padding {
    pub monitor: u64,
    pub workspace: u64,
    pub padding: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Bordered,
    Floating,
    Managed,
    NameChange,
    Tray,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Rule {
    Class { name: String },
    Exe { name: String },
    Title { name: String },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct Window {
    pub categories: Vec<Category>,
    #[serde(rename = "rule")]
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Konfig {
    pub container_padding: Option<Padding>,
    pub workspace_padding: Option<Padding>,
    pub keys: HashMap<Message, VirtualKey>,
    #[serde(rename = "window")]
    pub windows: Vec<Window>,
}

/// Executes a `komorebic` command and immediately
/// returns the status of the child process.
macro_rules! komorebic {
    ($($args:expr), +) => {
        std::process::Command::new("komorebic")
            .args([$($args), +])
            .status()
    };
}

#[rustfmt::skip]
pub(crate) use komorebic;
