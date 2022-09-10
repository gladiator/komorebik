use eyre::{
    eyre,
    Result,
};
use num_traits::{
    FromPrimitive,
    ToPrimitive,
};
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

/// Attempts to register a system-wide hotkey that will cause our
/// application to receive [MSG]s that occur when certain keys are/have
/// been pressed.
pub(crate) fn register_hot_key(key: VirtualKey) -> Result<()> {
    let message = key.to_i32().ok_or(eyre!("invalid virtual key"))?;
    let key = key as u32;
    unsafe { RegisterHotKey(None, message, MOD_ALT | MOD_CONTROL | MOD_NOREPEAT, key).ok()? };
    Ok(())
}

/// Attempts to unregister a system-wide hotkey based on the key itself.
pub(crate) fn unregister_hot_key(key: VirtualKey) -> Result<()> {
    unsafe { UnregisterHotKey(None, key.to_i32().ok_or(eyre!("unknown key"))?) };
    Ok(())
}

/// Polls the system using [GetMessageA], this blocks.  Will
/// cause the current thread to wait for any registered messages
/// that relate to our keys.
pub(crate) fn poll_key() -> Result<Option<VirtualKey>> {
    let result = unsafe {
        let mut msg = MSG::default();
        GetMessageA(&mut msg, None, WM_HOTKEY, WM_HOTKEY).ok()?;
        msg
    };

    // We only want to pass messages that directly relate to
    // our hotkeys.  The range filters in `GetMessageA` should
    // make this redundant?
    if result.message != WM_HOTKEY {
        return Ok(None);
    }

    // Try to parse the "message" from the `wParam` field
    Ok(VirtualKey::from_usize(result.wParam.0))
}
