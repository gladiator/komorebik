use ::config::{
    Config,
    File,
};
use anyhow::Result;

use crate::{
    config::{
        komorebic,
        Category,
        Konfig,
        Rule,
    },
    keyboard::HotKey,
    message::Message,
    system::poll_message,
};

mod config;
mod keyboard;
mod message;
mod system;

/// Initializes the user's configuration via `komorebic`.
fn init(config: Konfig) -> Result<Vec<HotKey>> {
    for window in config.windows {
        for rule in &window.rules {
            let (rule, name) = match rule {
                Rule::Class { name } => ("class", name),
                Rule::Exe { name } => ("exe", name),
                Rule::Title { name } => ("title", name),
            };

            for category in &window.categories {
                let name = format!("\"{}\"", name);
                komorebic!(
                    match category {
                        Category::Bordered => "identify-border-overflow-application",
                        Category::Floating => "float-rule",
                        Category::Managed => "manage-rule",
                        Category::NameChange => "identify-object-name-change-application",
                        Category::Tray => "identify-tray-application",
                    },
                    rule,
                    &name
                )?;
            }
        }
    }

    if let Some(container) = config.container_padding {
        let m = container.monitor.to_string();
        let w = container.workspace.to_string();
        let p = container.padding.to_string();
        komorebic!("container-padding", &m, &w, &p)?;
    }

    if let Some(workspace) = config.workspace_padding {
        let m = workspace.monitor.to_string();
        let w = workspace.workspace.to_string();
        let p = workspace.padding.to_string();
        komorebic!("workspace-padding", &m, &w, &p)?;
    }

    // We store the keys in a Vec and hand it off to the
    // caller so they stay within their lifetimes during the
    // application's execution.
    let mut keys = Vec::new();
    for (message, key) in config.keys {
        keys.push(HotKey::register(message, key)?);
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

    let _keys = init(config)?;
    while let Some(message) = poll_message()? {
        if message == Message::Stop {
            // TODO: Should this execute `komorebic stop`?
            break;
        }

        match message {
            Message::FocusWindowLeft => komorebic!("focus", "left")?,
            Message::FocusWindowRight => komorebic!("focus", "right")?,
            Message::FocusWindowUp => komorebic!("focus", "up")?,
            Message::FocusWindowDown => komorebic!("focus", "down")?,

            Message::MoveWindowLeft => komorebic!("move", "left")?,
            Message::MoveWindowRight => komorebic!("move", "right")?,
            Message::MoveWindowUp => komorebic!("move", "up")?,
            Message::MoveWindowDown => komorebic!("move", "down")?,

            Message::ResizeWindowEdgeLeftDecrease => komorebic!("resize", "left", "decrease")?,
            Message::ResizeWindowEdgeLeftIncrease => komorebic!("resize", "left", "increase")?,
            Message::ResizeWindowEdgeRightDecrease => komorebic!("resize", "right", "decrease")?,
            Message::ResizeWindowEdgeRightIncrease => komorebic!("resize", "right", "increase")?,
            Message::ResizeWindowEdgeUpDecrease => komorebic!("resize", "up", "decrease")?,
            Message::ResizeWindowEdgeUpIncrease => komorebic!("resize", "up", "increase")?,
            Message::ResizeWindowEdgeDownDecrease => komorebic!("resize", "down", "decrease")?,
            Message::ResizeWindowEdgeDownIncrease => komorebic!("resize", "down", "increase")?,

            Message::ResizeWindowAxisHorizontalDecrease => {
                komorebic!("resize-axis", "horizontal", "decrease")?
            },
            Message::ResizeWindowAxisHorizontalIncrease => {
                komorebic!("resize-axis", "horizontal", "increase")?
            },
            Message::ResizeWindowAxisVerticalDecrease => {
                komorebic!("resize-axis", "vertical", "decrease")?
            },
            Message::ResizeWindowAxisVerticalIncrease => {
                komorebic!("resize-axis", "vertical", "increase")?
            },
            Message::ResizeWindowAxisHorizontalAndVerticalDecrease => {
                komorebic!("resize-axis", "horizontal_and_vertical", "decrease")?
            },
            Message::ResizeWindowAxisHorizontalAndVerticalIncrease => {
                komorebic!("resize-axis", "horizontal_and_vertical", "increase")?
            },
            _ => continue,
        };
    }

    Ok(())
}
