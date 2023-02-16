use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Write},
};

use crate::Position;

#[derive(Default, Debug, Clone, Copy)]
pub struct Line {
    start: usize,
    end: usize,
    pub idx: usize,
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
    pub fn open(filename: &String) -> Result<Self, Error> {
        if let Ok(data) = fs::read(filename) {
            let lines = Document::get_lines(&data);
            Ok(Self {
                filename: Some(filename.to_string()),
                data,
                lines,
            })
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Não foi possível abrir o arquivo",
            ))
        }
    }

    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            filename: None,
            data: Vec::new(),
            lines: Vec::from([Line::new(0, 0, 0)]),
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
    pub fn get_str_line(&self, index: usize) -> String {
        let l = self.lines.get(index).unwrap();
        String::from_utf8_lossy(&self.data[l.start..l.end]).to_string()
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
        let index = pos.x + self.data[..l.start].len();
        self.data.insert(index, c as u8);
        self.lines = Document::get_lines(&self.data);
        Ok(())
    }

    pub fn delete(&mut self, pos: Position) -> Result<(), Error> {
        let l = self.lines[pos.y];
        let index = pos.x + self.data[..l.start].len();
        if index < self.data.len() {
            self.data.remove(index);
            self.lines = Document::get_lines(&self.data);
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), Error> {
        if let Some(filename) = &self.filename {
            let mut file = File::create(filename)?;
            file.write_all(self.data.as_slice())?;
        }
        Ok(())
    }
}
