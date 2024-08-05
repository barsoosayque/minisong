use bevy::{
    input::{
        keyboard::{KeyboardInput, NativeKeyCode},
        ButtonState,
    },
    prelude::*,
};
use crossterm::event::KeyCode as CrosstermKeyCode;
use crossterm::event::MediaKeyCode as CrosstermMediaKeyCode;
use crossterm::event::ModifierKeyCode as CrosstermModifierKeyCode;

use std::time::Duration;

pub fn ui_input_system(mut input_event_writer: EventWriter<KeyboardInput>) {
    use crossterm::event;

    while event::poll(Duration::default()).unwrap() {
        match event::read().unwrap() {
            event::Event::Key(key) => {
                let logical_key = convert_logical_key(&key.code);
                let key_code = convert_physical_key(&key.code);
                let window = Entity::PLACEHOLDER;
                let state = match key.kind {
                    event::KeyEventKind::Press => ButtonState::Pressed,
                    event::KeyEventKind::Repeat => ButtonState::Pressed,
                    event::KeyEventKind::Release => ButtonState::Released,
                };
                input_event_writer.send(KeyboardInput { key_code, logical_key, state, window });
            },
            _ => {},
        };
    }
}

fn convert_physical_key(code: &CrosstermKeyCode) -> KeyCode {
    match code {
        CrosstermKeyCode::Backspace => KeyCode::Backspace,
        CrosstermKeyCode::Enter => KeyCode::Enter,
        CrosstermKeyCode::Left => KeyCode::ArrowLeft,
        CrosstermKeyCode::Right => KeyCode::ArrowRight,
        CrosstermKeyCode::Up => KeyCode::ArrowUp,
        CrosstermKeyCode::Down => KeyCode::ArrowDown,
        CrosstermKeyCode::Home => KeyCode::Home,
        CrosstermKeyCode::End => KeyCode::End,
        CrosstermKeyCode::PageUp => KeyCode::PageUp,
        CrosstermKeyCode::PageDown => KeyCode::PageDown,
        CrosstermKeyCode::Tab => KeyCode::Tab,
        CrosstermKeyCode::BackTab => KeyCode::Tab,
        CrosstermKeyCode::Delete => KeyCode::Delete,
        CrosstermKeyCode::Insert => KeyCode::Insert,
        CrosstermKeyCode::Esc => KeyCode::Escape,
        CrosstermKeyCode::CapsLock => KeyCode::CapsLock,
        CrosstermKeyCode::ScrollLock => KeyCode::ScrollLock,
        CrosstermKeyCode::NumLock => KeyCode::NumLock,
        CrosstermKeyCode::PrintScreen => KeyCode::PrintScreen,
        CrosstermKeyCode::Pause => KeyCode::Pause,
        CrosstermKeyCode::Menu => KeyCode::ContextMenu,
        CrosstermKeyCode::F(1) => KeyCode::F1,
        CrosstermKeyCode::F(2) => KeyCode::F2,
        CrosstermKeyCode::F(3) => KeyCode::F3,
        CrosstermKeyCode::F(4) => KeyCode::F4,
        CrosstermKeyCode::F(5) => KeyCode::F5,
        CrosstermKeyCode::F(6) => KeyCode::F6,
        CrosstermKeyCode::F(7) => KeyCode::F7,
        CrosstermKeyCode::F(8) => KeyCode::F8,
        CrosstermKeyCode::F(9) => KeyCode::F9,
        CrosstermKeyCode::F(10) => KeyCode::F10,
        CrosstermKeyCode::F(11) => KeyCode::F11,
        CrosstermKeyCode::F(12) => KeyCode::F12,
        CrosstermKeyCode::F(13) => KeyCode::F13,
        CrosstermKeyCode::F(14) => KeyCode::F14,
        CrosstermKeyCode::F(15) => KeyCode::F15,
        CrosstermKeyCode::F(16) => KeyCode::F16,
        CrosstermKeyCode::F(17) => KeyCode::F17,
        CrosstermKeyCode::F(18) => KeyCode::F18,
        CrosstermKeyCode::F(19) => KeyCode::F19,
        CrosstermKeyCode::F(20) => KeyCode::F20,
        CrosstermKeyCode::F(21) => KeyCode::F21,
        CrosstermKeyCode::F(22) => KeyCode::F22,
        CrosstermKeyCode::F(23) => KeyCode::F23,
        CrosstermKeyCode::F(24) => KeyCode::F24,
        CrosstermKeyCode::F(25) => KeyCode::F25,
        CrosstermKeyCode::F(26) => KeyCode::F26,
        CrosstermKeyCode::F(27) => KeyCode::F27,
        CrosstermKeyCode::F(28) => KeyCode::F28,
        CrosstermKeyCode::F(29) => KeyCode::F29,
        CrosstermKeyCode::F(30) => KeyCode::F30,
        CrosstermKeyCode::F(31) => KeyCode::F31,
        CrosstermKeyCode::F(32) => KeyCode::F32,
        CrosstermKeyCode::F(33) => KeyCode::F33,
        CrosstermKeyCode::F(34) => KeyCode::F34,
        CrosstermKeyCode::F(35) => KeyCode::F35,
        CrosstermKeyCode::Char('`') => KeyCode::Backquote,
        CrosstermKeyCode::Char('\\') => KeyCode::Backslash,
        CrosstermKeyCode::Char('[') => KeyCode::BracketLeft,
        CrosstermKeyCode::Char(']') => KeyCode::BracketRight,
        CrosstermKeyCode::Char(',') => KeyCode::Comma,
        CrosstermKeyCode::Char('0') => KeyCode::Digit0,
        CrosstermKeyCode::Char('1') => KeyCode::Digit1,
        CrosstermKeyCode::Char('2') => KeyCode::Digit2,
        CrosstermKeyCode::Char('3') => KeyCode::Digit3,
        CrosstermKeyCode::Char('4') => KeyCode::Digit4,
        CrosstermKeyCode::Char('5') => KeyCode::Digit5,
        CrosstermKeyCode::Char('6') => KeyCode::Digit6,
        CrosstermKeyCode::Char('7') => KeyCode::Digit7,
        CrosstermKeyCode::Char('8') => KeyCode::Digit8,
        CrosstermKeyCode::Char('9') => KeyCode::Digit9,
        CrosstermKeyCode::Char('=') => KeyCode::Equal,
        CrosstermKeyCode::Char('a') => KeyCode::KeyA,
        CrosstermKeyCode::Char('b') => KeyCode::KeyB,
        CrosstermKeyCode::Char('c') => KeyCode::KeyC,
        CrosstermKeyCode::Char('d') => KeyCode::KeyD,
        CrosstermKeyCode::Char('e') => KeyCode::KeyE,
        CrosstermKeyCode::Char('f') => KeyCode::KeyF,
        CrosstermKeyCode::Char('g') => KeyCode::KeyG,
        CrosstermKeyCode::Char('h') => KeyCode::KeyH,
        CrosstermKeyCode::Char('i') => KeyCode::KeyI,
        CrosstermKeyCode::Char('j') => KeyCode::KeyJ,
        CrosstermKeyCode::Char('k') => KeyCode::KeyK,
        CrosstermKeyCode::Char('l') => KeyCode::KeyL,
        CrosstermKeyCode::Char('m') => KeyCode::KeyM,
        CrosstermKeyCode::Char('n') => KeyCode::KeyN,
        CrosstermKeyCode::Char('o') => KeyCode::KeyO,
        CrosstermKeyCode::Char('p') => KeyCode::KeyP,
        CrosstermKeyCode::Char('q') => KeyCode::KeyQ,
        CrosstermKeyCode::Char('r') => KeyCode::KeyR,
        CrosstermKeyCode::Char('s') => KeyCode::KeyS,
        CrosstermKeyCode::Char('t') => KeyCode::KeyT,
        CrosstermKeyCode::Char('u') => KeyCode::KeyU,
        CrosstermKeyCode::Char('v') => KeyCode::KeyV,
        CrosstermKeyCode::Char('w') => KeyCode::KeyW,
        CrosstermKeyCode::Char('x') => KeyCode::KeyX,
        CrosstermKeyCode::Char('y') => KeyCode::KeyY,
        CrosstermKeyCode::Char('z') => KeyCode::KeyZ,
        CrosstermKeyCode::Char('-') => KeyCode::Minus,
        CrosstermKeyCode::Char('.') => KeyCode::Period,
        CrosstermKeyCode::Char('\'') => KeyCode::Quote,
        CrosstermKeyCode::Char(';') => KeyCode::Semicolon,
        CrosstermKeyCode::Char('/') => KeyCode::Slash,
        CrosstermKeyCode::Media(media) => match media {
            CrosstermMediaKeyCode::Pause => KeyCode::Pause,
            CrosstermMediaKeyCode::PlayPause => KeyCode::MediaPlayPause,
            CrosstermMediaKeyCode::Stop => KeyCode::MediaStop,
            CrosstermMediaKeyCode::TrackNext => KeyCode::MediaTrackNext,
            CrosstermMediaKeyCode::TrackPrevious => KeyCode::MediaTrackPrevious,
            CrosstermMediaKeyCode::LowerVolume => KeyCode::AudioVolumeDown,
            CrosstermMediaKeyCode::RaiseVolume => KeyCode::AudioVolumeUp,
            CrosstermMediaKeyCode::MuteVolume => KeyCode::AudioVolumeMute,
            _ => KeyCode::Unidentified(NativeKeyCode::Unidentified),
        },
        CrosstermKeyCode::Modifier(modifier) => match modifier {
            CrosstermModifierKeyCode::LeftShift => KeyCode::ShiftLeft,
            CrosstermModifierKeyCode::LeftControl => KeyCode::ControlLeft,
            CrosstermModifierKeyCode::LeftAlt => KeyCode::AltLeft,
            CrosstermModifierKeyCode::LeftSuper => KeyCode::SuperLeft,
            CrosstermModifierKeyCode::LeftHyper => KeyCode::Hyper,
            CrosstermModifierKeyCode::LeftMeta => KeyCode::Meta,
            CrosstermModifierKeyCode::RightShift => KeyCode::ShiftRight,
            CrosstermModifierKeyCode::RightControl => KeyCode::ControlRight,
            CrosstermModifierKeyCode::RightAlt => KeyCode::AltRight,
            CrosstermModifierKeyCode::RightSuper => KeyCode::SuperRight,
            CrosstermModifierKeyCode::RightHyper => KeyCode::Hyper,
            CrosstermModifierKeyCode::RightMeta => KeyCode::Meta,
            _ => KeyCode::Unidentified(NativeKeyCode::Unidentified),
        },
        _ => KeyCode::Unidentified(NativeKeyCode::Unidentified),
    }
}

fn convert_logical_key(code: &CrosstermKeyCode) -> bevy::input::keyboard::Key {
    match code {
        CrosstermKeyCode::Backspace => bevy::input::keyboard::Key::Backspace,
        CrosstermKeyCode::Enter => bevy::input::keyboard::Key::Enter,
        CrosstermKeyCode::Left => bevy::input::keyboard::Key::ArrowLeft,
        CrosstermKeyCode::Right => bevy::input::keyboard::Key::ArrowRight,
        CrosstermKeyCode::Up => bevy::input::keyboard::Key::ArrowUp,
        CrosstermKeyCode::Down => bevy::input::keyboard::Key::ArrowDown,
        CrosstermKeyCode::Home => bevy::input::keyboard::Key::Home,
        CrosstermKeyCode::End => bevy::input::keyboard::Key::End,
        CrosstermKeyCode::PageUp => bevy::input::keyboard::Key::PageUp,
        CrosstermKeyCode::PageDown => bevy::input::keyboard::Key::PageDown,
        CrosstermKeyCode::Tab => bevy::input::keyboard::Key::Tab,
        CrosstermKeyCode::Delete => bevy::input::keyboard::Key::Delete,
        CrosstermKeyCode::Insert => bevy::input::keyboard::Key::Insert,
        CrosstermKeyCode::Esc => bevy::input::keyboard::Key::Escape,
        CrosstermKeyCode::CapsLock => bevy::input::keyboard::Key::CapsLock,
        CrosstermKeyCode::ScrollLock => bevy::input::keyboard::Key::ScrollLock,
        CrosstermKeyCode::NumLock => bevy::input::keyboard::Key::NumLock,
        CrosstermKeyCode::PrintScreen => bevy::input::keyboard::Key::PrintScreen,
        CrosstermKeyCode::Pause => bevy::input::keyboard::Key::Pause,
        CrosstermKeyCode::Menu => bevy::input::keyboard::Key::ContextMenu,
        CrosstermKeyCode::F(1) => bevy::input::keyboard::Key::F1,
        CrosstermKeyCode::F(2) => bevy::input::keyboard::Key::F2,
        CrosstermKeyCode::F(3) => bevy::input::keyboard::Key::F3,
        CrosstermKeyCode::F(4) => bevy::input::keyboard::Key::F4,
        CrosstermKeyCode::F(5) => bevy::input::keyboard::Key::F5,
        CrosstermKeyCode::F(6) => bevy::input::keyboard::Key::F6,
        CrosstermKeyCode::F(7) => bevy::input::keyboard::Key::F7,
        CrosstermKeyCode::F(8) => bevy::input::keyboard::Key::F8,
        CrosstermKeyCode::F(9) => bevy::input::keyboard::Key::F9,
        CrosstermKeyCode::F(10) => bevy::input::keyboard::Key::F10,
        CrosstermKeyCode::F(11) => bevy::input::keyboard::Key::F11,
        CrosstermKeyCode::F(12) => bevy::input::keyboard::Key::F12,
        CrosstermKeyCode::F(13) => bevy::input::keyboard::Key::F13,
        CrosstermKeyCode::F(14) => bevy::input::keyboard::Key::F14,
        CrosstermKeyCode::F(15) => bevy::input::keyboard::Key::F15,
        CrosstermKeyCode::F(16) => bevy::input::keyboard::Key::F16,
        CrosstermKeyCode::F(17) => bevy::input::keyboard::Key::F17,
        CrosstermKeyCode::F(18) => bevy::input::keyboard::Key::F18,
        CrosstermKeyCode::F(19) => bevy::input::keyboard::Key::F19,
        CrosstermKeyCode::F(20) => bevy::input::keyboard::Key::F20,
        CrosstermKeyCode::F(21) => bevy::input::keyboard::Key::F21,
        CrosstermKeyCode::F(22) => bevy::input::keyboard::Key::F22,
        CrosstermKeyCode::F(23) => bevy::input::keyboard::Key::F23,
        CrosstermKeyCode::F(24) => bevy::input::keyboard::Key::F24,
        CrosstermKeyCode::F(25) => bevy::input::keyboard::Key::F25,
        CrosstermKeyCode::F(26) => bevy::input::keyboard::Key::F26,
        CrosstermKeyCode::F(27) => bevy::input::keyboard::Key::F27,
        CrosstermKeyCode::F(28) => bevy::input::keyboard::Key::F28,
        CrosstermKeyCode::F(29) => bevy::input::keyboard::Key::F29,
        CrosstermKeyCode::F(30) => bevy::input::keyboard::Key::F30,
        CrosstermKeyCode::F(31) => bevy::input::keyboard::Key::F31,
        CrosstermKeyCode::F(32) => bevy::input::keyboard::Key::F32,
        CrosstermKeyCode::F(33) => bevy::input::keyboard::Key::F33,
        CrosstermKeyCode::F(34) => bevy::input::keyboard::Key::F34,
        CrosstermKeyCode::F(35) => bevy::input::keyboard::Key::F35,
        CrosstermKeyCode::Char(char) => {
            bevy::input::keyboard::Key::Character(char.to_string().into())
        },
        CrosstermKeyCode::Media(media) => match media {
            CrosstermMediaKeyCode::Play => bevy::input::keyboard::Key::Play,
            CrosstermMediaKeyCode::Pause => bevy::input::keyboard::Key::Pause,
            CrosstermMediaKeyCode::PlayPause => bevy::input::keyboard::Key::MediaPlayPause,
            CrosstermMediaKeyCode::Stop => bevy::input::keyboard::Key::MediaStop,
            CrosstermMediaKeyCode::FastForward => bevy::input::keyboard::Key::MediaFastForward,
            CrosstermMediaKeyCode::Rewind => bevy::input::keyboard::Key::MediaRewind,
            CrosstermMediaKeyCode::TrackNext => bevy::input::keyboard::Key::MediaTrackNext,
            CrosstermMediaKeyCode::TrackPrevious => bevy::input::keyboard::Key::MediaTrackPrevious,
            CrosstermMediaKeyCode::Record => bevy::input::keyboard::Key::MediaRecord,
            CrosstermMediaKeyCode::LowerVolume => bevy::input::keyboard::Key::AudioVolumeDown,
            CrosstermMediaKeyCode::RaiseVolume => bevy::input::keyboard::Key::AudioVolumeUp,
            CrosstermMediaKeyCode::MuteVolume => bevy::input::keyboard::Key::AudioVolumeMute,
            _ => bevy::input::keyboard::Key::Unidentified(
                bevy::input::keyboard::NativeKey::Unidentified,
            ),
        },
        CrosstermKeyCode::Modifier(modifier) => match modifier {
            CrosstermModifierKeyCode::IsoLevel3Shift
            | CrosstermModifierKeyCode::IsoLevel5Shift
            | CrosstermModifierKeyCode::RightShift
            | CrosstermModifierKeyCode::LeftShift => bevy::input::keyboard::Key::Shift,
            CrosstermModifierKeyCode::RightControl | CrosstermModifierKeyCode::LeftControl => {
                bevy::input::keyboard::Key::Control
            },
            CrosstermModifierKeyCode::RightAlt | CrosstermModifierKeyCode::LeftAlt => {
                bevy::input::keyboard::Key::Alt
            },
            CrosstermModifierKeyCode::RightSuper | CrosstermModifierKeyCode::LeftSuper => {
                bevy::input::keyboard::Key::Super
            },
            CrosstermModifierKeyCode::RightHyper | CrosstermModifierKeyCode::LeftHyper => {
                bevy::input::keyboard::Key::Hyper
            },
            CrosstermModifierKeyCode::RightMeta | CrosstermModifierKeyCode::LeftMeta => {
                bevy::input::keyboard::Key::Meta
            },
        },
        _ => {
            bevy::input::keyboard::Key::Unidentified(bevy::input::keyboard::NativeKey::Unidentified)
        },
    }
}
