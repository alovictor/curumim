#[derive(Default, Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    #[must_use]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn from_rect(rect: &Rect) -> Self {
        Self {
            x: rect.tl.x,
            y: rect.tl.y,
        }
    }

    #[must_use]
    pub fn in_range(&self, rect: &Rect) -> bool {
        self.x >= rect.tl.x && self.x <= rect.br.x && self.y >= rect.tl.y && self.y <= rect.br.y
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Rect {
    pub tl: Position,
    pub br: Position,
}

impl Rect {
    #[must_use]
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self {
            tl: Position { x: x1, y: y1 },
            br: Position { x: x2, y: y2 },
        }
    }
}
