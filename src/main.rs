use std::{
    ffi::OsStr,
    process::Command,
};

use anyhow::Result;

use crate::{
    config::{
        Category,
        Config,
        Rule,
    },
    keyboard::HotKey,
    message::{
        poll_message,
        Message,
    },
};

mod config;
mod keyboard;
mod message;

fn komorebic<A, S>(arguments: A) -> Result<()>
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("komorebic").args(arguments).status()?;
    Ok(())
}

fn init(config: Config) -> Result<Vec<HotKey>> {
    if let Some(window) = config.window.as_ref() {
        for window in window {
            for rule in &window.rule {
                let (rule, name) = match rule {
                    Rule::Class { name } => ("class", name),
                    Rule::Exe { name } => ("exe", name),
                    Rule::Title { name } => ("title", name),
                };

                for category in &window.categories {
                    let sub = match category {
                        Category::Float => "float-rule",
                        Category::Manage => "manage-rule",
                        Category::NameChange => "identify-object-name-change-application",
                        Category::Overflow => "identify-border-overflow-application",
                        Category::Tray => "identify-tray-application",
                    };

                    komorebic([sub, rule, &name])?;
                }
            }
        }
    }

    if let Some(container_padding) = config.container_padding {
        let monitor = container_padding.monitor.to_string();
        let workspace = container_padding.workspace.to_string();
        let padding = container_padding.padding.to_string();
        komorebic(["container-padding", &monitor, &workspace, &padding])?;
    }

    if let Some(workspace_padding) = config.workspace_padding {
        let monitor = workspace_padding.monitor.to_string();
        let workspace = workspace_padding.workspace.to_string();
        let padding = workspace_padding.padding.to_string();
        komorebic(["workspace-padding", &monitor, &workspace, &padding])?;
    }

    let mut keys = Vec::new();
    if let Some(bindings) = config.key_bindings {
        if let Some(key) = bindings.quit {
            keys.push(HotKey::new(Message::Quit, key));
        }

        if let Some(axis_decrease) = bindings.axis_decrease {
            if let Some(key) = axis_decrease.horizontal {
                keys.push(HotKey::new(Message::DecreaseHorizontal, key));
            }

            if let Some(key) = axis_decrease.vertical {
                keys.push(HotKey::new(Message::DecreaseVertical, key));
            }

            if let Some(key) = axis_decrease.horizontal_vertical {
                keys.push(HotKey::new(Message::DecreaseHorizontalVertical, key));
            }
        }

        if let Some(axis_increase) = bindings.axis_increase {
            if let Some(key) = axis_increase.horizontal {
                keys.push(HotKey::new(Message::IncreaseHorizontal, key));
            }

            if let Some(key) = axis_increase.vertical {
                keys.push(HotKey::new(Message::IncreaseVertical, key));
            }

            if let Some(key) = axis_increase.horizontal_vertical {
                keys.push(HotKey::new(Message::IncreaseHorizontalVertical, key));
            }
        }

        if let Some(move_window) = bindings.move_window {
            if let Some(key) = move_window.left {
                keys.push(HotKey::new(Message::MoveLeft, key));
            }

            if let Some(key) = move_window.right {
                keys.push(HotKey::new(Message::MoveRight, key));
            }

            if let Some(key) = move_window.up {
                keys.push(HotKey::new(Message::MoveUp, key));
            }

            if let Some(key) = move_window.down {
                keys.push(HotKey::new(Message::MoveDown, key));
            }
        }

        for key in &keys {
            key.register()?;
        }
    }

    Ok(keys)
}

fn poll() -> Result<Option<Message>> {
    if let Some(message) = poll_message()? {
        return Ok(match message {
            Message::Quit => None,
            _ => Some(message),
        });
    }

    Ok(None)
}

fn main() -> Result<()> {
    let config = if let Some(dir) = dirs::home_dir() {
        dir.join(".config/").join("komorebik.toml")
    } else {
        panic!("Unknown configuration location!");
    };

    let config = std::fs::read(config)?;
    let config = toml::from_slice(config.as_slice())?;
    let _keys = init(config)?;
    while let Some(message) = poll()? {
        match message {
            Message::DecreaseVertical => komorebic(["resize-axis", "vertical", "decrease"])?,
            Message::IncreaseVertical => komorebic(["resize-axis", "vertical", "increase"])?,
            Message::DecreaseHorizontal => komorebic(["resize-axis", "horizontal", "decrease"])?,
            Message::IncreaseHorizontal => komorebic(["resize-axis", "horizontal", "increase"])?,
            Message::MoveLeft => komorebic(["move", "left"])?,
            Message::MoveRight => komorebic(["move", "right"])?,
            Message::MoveUp => komorebic(["move", "up"])?,
            Message::MoveDown => komorebic(["move", "down"])?,
            _ => continue,
        }
    }

    Ok(())
}
