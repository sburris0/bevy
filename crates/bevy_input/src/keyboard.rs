use crate::{ElementState, Input};
use bevy_app::prelude::*;
use bevy_ecs::ResMut;
use std::convert::TryInto;

/// A key input event from a keyboard device
#[derive(Debug, Clone)]
pub struct KeyboardInput {
    pub scan_code: u32,
    pub key_code: Option<KeyCode>,
    pub state: ElementState,
}

/// Updates the Input<KeyCode> resource with the latest KeyboardInput events
pub fn keyboard_input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    keyboard_input.update();
    for event in keyboard_input_events.iter() {
        if let KeyboardInput {
            key_code: Some(key_code),
            state,
            ..
        } = event
        {
            match state {
                ElementState::Pressed => keyboard_input.press(*key_code),
                ElementState::Released => keyboard_input.release(*key_code),
            }
        }
    }
}

/// The key code of a keyboard input.
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum KeyCode {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    NumpadAdd,
    Apostrophe,
    Apps,
    Asterisk,
    Plus,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    NumpadDecimal,
    NumpadDivide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    NumpadMultiply,
    Mute,
    MyComputer,
    NavigateForward,  // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    NumpadSubtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

impl TryInto<char> for KeyCode {
    type Error = ();

    fn try_into(self) -> Result<char, Self::Error> {
        match self {
            KeyCode::Key1 | KeyCode::Numpad1 => Ok('1'),
            KeyCode::Key2 | KeyCode::Numpad2 => Ok('2'),
            KeyCode::Key3 | KeyCode::Numpad3 => Ok('3'),
            KeyCode::Key4 | KeyCode::Numpad4 => Ok('4'),
            KeyCode::Key5 | KeyCode::Numpad5 => Ok('5'),
            KeyCode::Key6 | KeyCode::Numpad6 => Ok('6'),
            KeyCode::Key7 | KeyCode::Numpad7 => Ok('7'),
            KeyCode::Key8 | KeyCode::Numpad8 => Ok('8'),
            KeyCode::Key9 | KeyCode::Numpad9 => Ok('9'),
            KeyCode::Key0 | KeyCode::Numpad0 => Ok('0'),
            KeyCode::A => Ok('a'),
            KeyCode::B => Ok('b'),
            KeyCode::C => Ok('c'),
            KeyCode::D => Ok('d'),
            KeyCode::E => Ok('e'),
            KeyCode::F => Ok('f'),
            KeyCode::G => Ok('g'),
            KeyCode::H => Ok('h'),
            KeyCode::I => Ok('i'),
            KeyCode::J => Ok('j'),
            KeyCode::K => Ok('k'),
            KeyCode::L => Ok('l'),
            KeyCode::M => Ok('m'),
            KeyCode::N => Ok('n'),
            KeyCode::O => Ok('o'),
            KeyCode::P => Ok('p'),
            KeyCode::Q => Ok('q'),
            KeyCode::R => Ok('r'),
            KeyCode::S => Ok('s'),
            KeyCode::T => Ok('t'),
            KeyCode::U => Ok('u'),
            KeyCode::V => Ok('v'),
            KeyCode::W => Ok('w'),
            KeyCode::X => Ok('x'),
            KeyCode::Y => Ok('y'),
            KeyCode::Z => Ok('z'),
            KeyCode::Caret => Ok('^'),
            KeyCode::Apostrophe => Ok('\''),
            KeyCode::Asterisk | KeyCode::NumpadMultiply => Ok('*'),
            KeyCode::Plus | KeyCode::NumpadAdd => Ok('+'),
            KeyCode::At => Ok('@'),
            KeyCode::Backslash => Ok('\\'),
            KeyCode::Colon => Ok(':'),
            KeyCode::Comma | KeyCode::NumpadComma => Ok(','),
            KeyCode::Period | KeyCode::NumpadDecimal => Ok('.'),
            KeyCode::Slash | KeyCode::NumpadDivide => Ok('/'),
            KeyCode::Equals | KeyCode::NumpadEquals => Ok('='),
            KeyCode::Grave => Ok('`'),
            KeyCode::Minus | KeyCode::NumpadSubtract => Ok('-'),
            KeyCode::Semicolon => Ok(';'),
            KeyCode::Yen => Ok('¥'),
            _ => Err(()),
        }
    }
}
