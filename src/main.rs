use ::config::{
    Config,
    File,
};
use eyre::Result;
use komorebi_core::SocketMessage;

use crate::{
    config::Konfig,
    keyboard::HotKey,
    system::{
        poll_key,
        process,
    },
};

mod config;
mod keyboard;
mod system;

/// Initializes the user's configuration via `komorebic`.
fn init(config: &Konfig) -> Result<Vec<HotKey>> {
    for window in &config.windows {
        for rule in &window.rules {
            if let Some(true) = &window.bordered {
                process(&SocketMessage::IdentifyBorderOverflowApplication(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.floating {
                process(&SocketMessage::FloatRule(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.layered {
                process(&SocketMessage::IdentifyLayeredApplication(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.managed {
                process(&SocketMessage::ManageRule(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.name_change {
                process(&SocketMessage::IdentifyObjectNameChangeApplication(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.tray {
                process(&SocketMessage::IdentifyTrayApplication(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }
        }
    }

    for option in &config.default {
        process(option)?;
    }

    let mut keys = Vec::new();
    for (key, _) in &config.keys {
        keys.push(HotKey::register(key.clone())?);
    }

    Ok(keys)
}

fn main() -> Result<()> {
    let config = dirs::home_dir()
        .expect("missing home directory!")
        .join(".config/")
        .join("komorebik.toml");

    let config = Config::builder()
        .add_source(File::from(config))
        .build()?
        .try_deserialize()?;

    let _keys = init(&config)?;
    while let Some(key) = poll_key()? {
        if let Some(message) = config.keys.get(&key) {
            process(message)?;
        }
    }

    Ok(())
}
