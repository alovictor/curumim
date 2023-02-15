#![warn(clippy::all, clippy::pedantic)]

mod document;
mod editor;
mod input;
mod term;
mod utils;

pub use document::{Document, Line};
use editor::Editor;
pub use input::{Input, Key};
pub use term::Term;
pub use utils::{Position, Rect};

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
