use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::PathBuf,
};

use eyre::Result;
use komorebi_core::{
    config_generation::{
        ApplicationConfiguration,
        ApplicationOptions,
    },
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
    pub name_change: Option<bool>,
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

pub(crate) fn gen_app_specific_cfg(path: PathBuf) -> Result<()> {
    let apps: Vec<ApplicationConfiguration> =
        serde_yaml::from_slice(std::fs::read(path)?.as_slice())?;
    let mut lines = vec![
        String::from("# Generated by komorebik.exe"),
        String::from("# To use this file, copy everything to the bottom of your existing komorebik config file"),
        String::new(),
    ];

    for app in apps {
        lines.push(String::from("[[window]]"));
        if let Some(options) = app.options {
            for opt in options {
                match opt {
                    ApplicationOptions::ObjectNameChange => {
                        lines.push(String::from("name_change = true"));
                    },
                    ApplicationOptions::Layered => {
                        lines.push(String::from("layered = true"));
                    },
                    ApplicationOptions::BorderOverflow => {
                        lines.push(String::from("bordered = true"));
                    },
                    ApplicationOptions::TrayAndMultiWindow => {
                        lines.push(String::from("# If you have disabled minimize/close to tray for this application, you can delete/comment out the next line"));
                        lines.push(String::from("tray = true"));
                    },
                    ApplicationOptions::Force => {
                        lines.push(String::from("managed = true"));
                    },
                };
            }
        }

        let (kind, id) = (app.identifier.kind, app.identifier.id);

        let mut float_rules = vec![];
        if let Some(float_identifiers) = app.float_identifiers {
            for float in float_identifiers {
                let lk = core::mem::discriminant(&kind);
                let rk = core::mem::discriminant(&float.kind);
                if float.id == id && lk == rk {
                    lines.push(String::from("floating = true"));
                } else {
                    float_rules.push(String::new());
                    if let Some(comment) = float.comment {
                        float_rules.push(format!("# {}", comment));
                    }

                    float_rules.push(String::from("[[window]]"));
                    float_rules.push(String::from("floating = true"));
                    float_rules.push(String::from("[[window.rule]]"));
                    float_rules.push(format!("type = \"{}\"", float.kind));
                    float_rules.push(format!("name = \"{}\"", float.id));
                }
            }
        }

        lines.push(String::from("[[window.rule]]"));
        lines.push(format!("type = \"{}\"", kind));
        lines.push(format!("name = \"{}\"", id));
        for float_rule in float_rules {
            lines.push(float_rule);
        }

        lines.push(String::new());
    }

    let path = dirs::home_dir()
        .expect("missing home directory!")
        .join(".config")
        .join("komorebik.generated.toml");

    let mut file = File::create(&path)?;
    file.write_all(lines.join("\n").as_bytes())?;
    println!(
        "generated app-specific config at {}!",
        path.display().to_string().replace("/", "\\")
    );

    Ok(())
}
