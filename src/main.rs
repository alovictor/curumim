#![warn(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;
use simplelog::*;
use std::fs::File;

mod cursor;
mod document;
mod editor;
mod input;
mod term;
mod utils;

pub use cursor::Cursor;
pub use document::{Document, Line};
use editor::Editor;
pub use input::{Input, Key};
pub use term::Term;
pub use utils::{Position, Rect};

fn main() {
    let _ = WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("debug.log").unwrap(),
    );
    let mut editor = Editor::new();
    editor.run();
}
