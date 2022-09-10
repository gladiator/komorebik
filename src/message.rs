use anyhow::Result;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        GetMessageA,
        MSG,
        WM_HOTKEY,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Message {
    Quit,

    DecreaseHorizontal,
    IncreaseHorizontal,

    DecreaseVertical,
    IncreaseVertical,

    DecreaseHorizontalVertical,
    IncreaseHorizontalVertical,

    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

pub fn poll_message() -> Result<Option<Message>> {
    Ok(unsafe {
        let mut msg = MSG::default();
        GetMessageA(&mut msg, HWND::default(), 0, 0).ok()?;
        if msg.message != WM_HOTKEY {
            None
        } else {
            Some(std::mem::transmute(msg.wParam.0))
        }
    })
}
