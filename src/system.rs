use std::{
    io::Write,
    path::PathBuf,
};

use eyre::{
    eyre,
    Result,
};
use komorebi_core::SocketMessage;
use lazy_static::lazy_static;
use num_traits::{
    FromPrimitive,
    ToPrimitive,
};
use uds_windows::UnixStream;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        RegisterHotKey,
        UnregisterHotKey,
        MOD_ALT,
        MOD_CONTROL,
        MOD_NOREPEAT,
    },
    WindowsAndMessaging::{
        GetMessageA,
        MSG,
        WM_HOTKEY,
    },
};

use crate::keyboard::VirtualKey;

/// Writes a message to Komorebi's socket.
///
/// We shouldn't need to reconnect every message but it seems
/// that komorebi doesn't actively maintain connected clients and
/// will only read a single message before dropping it. :(
pub(crate) fn send_message(message: SocketMessage) -> Result<()> {
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

/// Attempts to register a system-wide hotkey that will cause our
/// application to receive [MSG]s that occur when certain keys are/have
/// been pressed.
pub(crate) fn register_hot_key(key: VirtualKey) -> Result<()> {
    let message = key.to_i32().ok_or(eyre!("invalid key"))?;
    let key = key as u32;
    unsafe { RegisterHotKey(None, message, MOD_ALT | MOD_CONTROL | MOD_NOREPEAT, key).ok()? };
    Ok(())
}

/// Attempts to unregister a system-wide hotkey based on the key itself.
pub(crate) fn unregister_hot_key(key: VirtualKey) -> Result<()> {
    unsafe { UnregisterHotKey(None, key.to_i32().ok_or(eyre!("invalid key"))?) };
    Ok(())
}

/// Polls the system using [GetMessageA].  Will block the current
/// thread to wait for any messages that relate to our keys.
pub(crate) fn poll_key() -> Result<Option<VirtualKey>> {
    let result = unsafe {
        let mut msg = MSG::default();
        GetMessageA(&mut msg, None, WM_HOTKEY, WM_HOTKEY).ok()?;
        msg
    };

    Ok(VirtualKey::from_usize(result.wParam.0))
}
