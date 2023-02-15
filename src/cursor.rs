use crate::Position;

pub struct Cursor {
    pos: Position,
}

impl Cursor {
    pub fn new(pos: Position) -> Self {
        Self { pos }
    }
}
