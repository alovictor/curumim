use std::{fs, io::Error};

#[derive(Default, Debug, Clone)]
pub struct Line {
    start: usize,
    end: usize,
    idx: usize,
}

impl Line {
    fn new(start: usize, end: usize, idx: usize) -> Self {
        Self { start, end, idx }
    }

    pub fn len(&self) -> usize {
        return self.end - self.start;
    }

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
        let mut data = fs::read(filename).expect("Não consegui abrir o arquivo");
        let lines = Document::get_lines(&mut data);
        Ok(Self {
            filename: Some(filename.to_string()),
            data,
            lines,
        })
    }

    fn get_lines(data: &mut Vec<u8>) -> Vec<Line> {
        let mut lines = Vec::new();
        let mut prev: usize = 0;
        let mut next: usize;
        for (idx, value) in data.iter_mut().enumerate() {
            if *value == '\n' as u8 {
                next = idx;
                lines.push(Line::new(prev, next, lines.len()));
                prev = next + 1;
            }
        }
        lines.push(Line::new(prev, data.len(), lines.len()));
        lines
    }

    pub fn get_line(&self, index: usize) -> Option<Line> {
        if let Some(l) = self.lines.get(index) {
            Some(l.clone())
        } else {
            None
        }
    }

    pub fn get_str_line(&self, index: usize) -> Option<String> {
        if let Some(l) = self.lines.get(index) {
            let line: String;
            if l.idx < 9 {
                line = format!("  {}", l.idx + 1);
            } else {
                line = format!(" {}", l.idx + 1);
            }

            Some(format!(
                "{} {}",
                line,
                String::from_utf8_lossy(&self.data[l.start..l.end])
            ))
        } else {
            None
        }
    }

    pub fn get_index(&self, l: &Line) -> usize {
        l.idx
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn has_at(&self, index: usize) -> bool {
        if let Some(_) = self.lines.get(index) {
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }
}
