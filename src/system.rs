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

/// Writes a message to komorebi's socket.
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

/// Attempts to register a system-wide hot-key.
pub(crate) fn register_hot_key(key: VirtualKey) -> Result<()> {
    let id = key.to_i32().ok_or(eyre!("invalid key"))?;
    unsafe { RegisterHotKey(None, id, MOD_ALT | MOD_CONTROL | MOD_NOREPEAT, key as u32).ok()? };
    Ok(())
}

/// Attempts to unregister a system-wide hot-key.
pub(crate) fn unregister_hot_key(key: VirtualKey) -> Result<()> {
    unsafe { UnregisterHotKey(None, key.to_i32().ok_or(eyre!("invalid key"))?) };
    Ok(())
}

/// Calls [GetMessageA] which polls the system for hot-key messages.
pub(crate) fn poll_key() -> Result<Option<VirtualKey>> {
    let result = unsafe {
        let mut msg = MSG::default();
        GetMessageA(&mut msg, None, WM_HOTKEY, WM_HOTKEY).ok()?;
        msg
    };

    Ok(VirtualKey::from_usize(result.wParam.0))
}
