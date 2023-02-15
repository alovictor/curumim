use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Char(char),
    F(u8),
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Tab,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Esc,
    MouseScrollDown,
    MouseScrollUp,
    Null,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub code: Key,
    pub modifiers: KeyModifiers,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            code: Key::Null,
            modifiers: KeyModifiers::NONE,
        }
    }
}

impl From<KeyEvent> for Input {
    /// Convert [`crossterm::event::KeyEvent`] to [`Input`].
    fn from(key: KeyEvent) -> Self {
        let modifiers = key.modifiers;
        let code = match key.code {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Enter => Key::Enter,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Tab => Key::Tab,
            KeyCode::Delete => Key::Delete,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::Esc => Key::Esc,
            KeyCode::F(x) => Key::F(x),
            _ => Key::Null,
        };
        Self { code, modifiers }
    }
}

impl From<MouseEvent> for Input {
    /// Convert [`crossterm::event::MouseEvent`] to [`Input`].
    fn from(mouse: MouseEvent) -> Self {
        let modifiers = mouse.modifiers;
        let code = match mouse.kind {
            MouseEventKind::ScrollDown => Key::MouseScrollDown,
            MouseEventKind::ScrollUp => Key::MouseScrollUp,
            _ => return Self::default(),
        };
        Self { code, modifiers }
    }
}
