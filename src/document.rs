use std::{fs, io::Error};

use crate::Position;

#[derive(Default, Debug, Clone, Copy)]
pub struct Line {
    start: usize,
    end: usize,
    idx: usize,
}

impl Line {
    #[must_use]
    pub fn new(start: usize, end: usize, idx: usize) -> Self {
        Self { start, end, idx }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    #[must_use]
    pub fn index(&self) -> usize {
        self.idx
    }
}

#[derive(Default, Clone)]
pub struct Document {
    pub filename: Option<String>,
    data: Vec<u8>,
    lines: Vec<Line>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let data = fs::read(filename)?;
        let lines = Document::get_lines(&data);
        Ok(Self {
            filename: Some(filename.to_string()),
            data,
            lines,
        })
    }

    fn get_lines(data: &Vec<u8>) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut prev: usize = 0;
        let mut next: usize;
        for (idx, value) in data.iter().enumerate() {
            if *value == b'\n' {
                next = idx;
                lines.push(Line::new(prev, next, lines.len()));
                prev = next + 1;
            }
        }
        lines.push(Line::new(prev, data.len(), lines.len()));
        lines
    }

    #[must_use]
    pub fn get_line(&self, index: usize) -> Option<Line> {
        self.lines.get(index).map(std::clone::Clone::clone)
    }

    #[must_use]
    pub fn get_str_line(&self, index: usize) -> Option<String> {
        if let Some(l) = self.lines.get(index) {
            let line: String = if l.idx < 9 {
                format!("  {}", l.idx + 1)
            } else {
                format!(" {}", l.idx + 1)
            };

            Some(format!(
                "{} {}",
                line,
                String::from_utf8_lossy(&self.data[l.start..l.end]).trim_end()
            ))
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_index(&self, l: &Line) -> usize {
        l.idx
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[must_use]
    pub fn has_at(&self, index: usize) -> bool {
        self.lines.get(index).is_some()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn insert(&mut self, pos: Position, c: char) -> Result<(), Error> {
        let l = self.lines[pos.y];
        let index = l.start + pos.x;
        self.data.insert(index, c as u8);
        Ok(())
    }
}
