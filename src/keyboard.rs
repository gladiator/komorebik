use eyre::Result;
use num_derive::{
    FromPrimitive,
    ToPrimitive,
};
use serde::Deserialize;

use crate::system::{
    register_hot_key,
    unregister_hot_key,
};

/// A system-wide key that is registered upon creation
/// and unregistered upon destruction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct HotKey(VirtualKey);

impl HotKey {
    /// Creates a [HotKey] and registers it to the system.
    /// Gives ownership of the key to whomever is calling, when
    /// it is dropped it will be unregistered from the system.
    pub fn register(key: VirtualKey) -> Result<Self> {
        register_hot_key(key)?;
        Ok(Self(key))
    }
}

impl Drop for HotKey {
    fn drop(&mut self) {
        // Unregister the key from the system, regardless
        unregister_hot_key(self.0).unwrap();
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, FromPrimitive, ToPrimitive)]
#[repr(u32)]
#[serde(rename_all = "snake_case")]
pub enum VirtualKey {
    LButton = 0x01,
    RButton = 0x02,
    Cancel = 0x03,
    MButton = 0x04,
    XButton1 = 0x05,
    XButton2 = 0x06,
    Back = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Return = 0x0D,
    Shift = 0x10,
    Control = 0x11,
    Menu = 0x12,
    Pause = 0x13,
    Capital = 0x14,
    Escape = 0x1B,
    Convert = 0x1C,
    NonConvert = 0x1D,
    Accept = 0x1E,
    ModeChange = 0x1F,
    Space = 0x20,
    Prior = 0x21,
    Next = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    Snapshot = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,
    #[serde(rename = "0")]
    Zero = 0x30,
    #[serde(rename = "1")]
    One = 0x31,
    #[serde(rename = "2")]
    Two = 0x32,
    #[serde(rename = "3")]
    Three = 0x33,
    #[serde(rename = "4")]
    Four = 0x34,
    #[serde(rename = "5")]
    Five = 0x35,
    #[serde(rename = "6")]
    Six = 0x36,
    #[serde(rename = "7")]
    Seven = 0x37,
    #[serde(rename = "8")]
    Eight = 0x38,
    #[serde(rename = "9")]
    Nine = 0x39,
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
    LWin = 0x5B,
    RWin = 0x5C,
    Apps = 0x5D,
    Sleep = 0x5F,
    #[serde(rename = "NumPad0")]
    NumPadZero = 0x60,
    #[serde(rename = "NumPad1")]
    NumPadOne = 0x61,
    #[serde(rename = "NumPad2")]
    NumPadTwo = 0x62,
    #[serde(rename = "NumPad3")]
    NumPadThree = 0x63,
    #[serde(rename = "NumPad4")]
    NumPadFour = 0x64,
    #[serde(rename = "NumPad5")]
    NumPadFive = 0x65,
    #[serde(rename = "NumPad6")]
    NumPadSix = 0x66,
    #[serde(rename = "NumPad7")]
    NumPadSeven = 0x67,
    #[serde(rename = "NumPad8")]
    NumPadEight = 0x68,
    #[serde(rename = "NumPad9")]
    NumPadNine = 0x69,
    #[serde(rename = "*")]
    Multiply = 0x6A,
    #[serde(rename = "+")]
    Add = 0x6B,
    Separator = 0x6C,
    #[serde(rename = "-")]
    Subtract = 0x6D,
    #[serde(rename = ".")]
    Decimal = 0x6E,
    #[serde(rename = "/")]
    Divide = 0x6F,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,
    NumLock = 0x90,
    Scroll = 0x91,
    LShift = 0xA0,
    RShift = 0xA1,
    LControl = 0xA2,
    RControl = 0xA3,
    LMenu = 0xA4,
    RMenu = 0xA5,
    BrowserBack = 0xA6,
    BrowserForward = 0xA7,
    BrowserRefresh = 0xA8,
    BrowserStop = 0xA9,
    BrowserSearch = 0xAA,
    BrowserFavorites = 0xAB,
    BrowserHome = 0xAC,
    VolumeMute = 0xAD,
    VolumeDown = 0xAE,
    VolumeUp = 0xAF,
    MediaNextTrack = 0xB0,
    MediaPrevTrack = 0xB1,
    MediaStop = 0xB2,
    MediaPlayPause = 0xB3,
    LaunchMail = 0xB4,
    LaunchMediaSelect = 0xB5,
    LaunchApp1 = 0xB6,
    LaunchApp2 = 0xB7,
}
