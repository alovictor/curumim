use std::error::Error;

pub use crate::editor::*;
pub use crate::terminal::*;

pub type CResult<T> = Result<T, Box<dyn Error>>;
pub type CBuffer = Vec<Cell>;
