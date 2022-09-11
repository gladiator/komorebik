use std::{
    io::Write,
    path::PathBuf,
};

use ::config::{
    Config,
    File,
};
use eyre::Result;
use komorebi_core::SocketMessage;
use lazy_static::lazy_static;
use uds_windows::UnixStream;

use crate::{
    config::Konfig,
    keyboard::HotKey,
    system::poll_key,
};

mod config;
mod keyboard;
mod system;

/// Submits a message over komorebi's socket.
fn process(message: &SocketMessage) -> Result<()> {
    lazy_static! {
        static ref SOCKET: PathBuf = dirs::data_local_dir()
            .expect("missing local data directory")
            .join("komorebi")
            .join("komorebi.sock");
    }

    let mut stream = UnixStream::connect(&(*SOCKET))?;
    stream.write_all(message.as_bytes()?.as_slice())?;
    Ok(())
}

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

            if let Some(true) = &window.object_name_change {
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

    if let Some(container) = &config.container_padding {
        process(&SocketMessage::ContainerPadding(
            container.monitor,
            container.workspace,
            container.padding,
        ))?;
    }

    if let Some(workspace) = &config.workspace_padding {
        process(&SocketMessage::WorkspacePadding(
            workspace.monitor,
            workspace.workspace,
            workspace.padding,
        ))?;
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
