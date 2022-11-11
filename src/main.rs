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
        send_message,
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
                send_message(SocketMessage::IdentifyBorderOverflowApplication(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.floating {
                send_message(SocketMessage::FloatRule(rule.identifier, rule.name.clone()))?;
            }

            if let Some(true) = &window.layered {
                send_message(SocketMessage::IdentifyLayeredApplication(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.managed {
                send_message(SocketMessage::ManageRule(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.name_change {
                send_message(SocketMessage::IdentifyObjectNameChangeApplication(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }

            if let Some(true) = &window.tray {
                send_message(SocketMessage::IdentifyTrayApplication(
                    rule.identifier,
                    rule.name.clone(),
                ))?;
            }
        }
    }

    for option in &config.default {
        send_message(option.clone())?;
    }

    let mut keys = Vec::new();
    for key in config.keys.keys() {
        keys.push(HotKey::new(*key));
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

    let keys = init(&config)?;
    for key in &keys {
        key.register();
    }

    while let Some(key) = poll_key()? {
        if let Some(message) = config.keys.get(&key) {
            send_message(message.clone())?;
        }
    }

    for key in &keys {
        key.unregister();
    }

    Ok(())
}
