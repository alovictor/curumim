use log::debug;
use ropey::{Rope, RopeSlice};
use std::{
    collections::VecDeque,
    fs::{self, File},
    path::PathBuf,
};

use crate::prelude::*;

#[derive(Clone)]
pub enum EditorCmd {
    InsertChar(usize, String),
    DeleteChar(usize, String),
}

pub struct Editor {
    path: PathBuf,
    text: Rope,
    cursor: usize,
    cmds: VecDeque<EditorCmd>,
}

impl Editor {
    pub fn new(_path: Option<PathBuf>) -> CResult<Self> {
        let path = match _path {
            Some(p) => p,
            None => PathBuf::default(),
        };

        Ok(Self {
            cursor: 0,
            text: Rope::from_str(
                &fs::read_to_string(path.clone()).or::<String>(Ok("".to_string()))?,
            ),
            path,
            cmds: VecDeque::default(),
        })
    }

    pub fn insert_char(&mut self, c: char) -> CResult<()> {
        self.text
            .insert_char(self.text.byte_to_char(self.cursor), c);
        self.move_cursor_h(1);
        Ok(())
    }

    pub fn delete_char(&mut self) -> CResult<()> {
        if self.cursor != 0 {
            self.text.remove(self.cursor - 1..self.cursor);
        }
        self.move_cursor_h(-1);
        Ok(())
    }

    pub fn save_file(&mut self) -> CResult<()> {
        self.text.write_to(File::create(&self.path)?)?;
        Ok(())
    }

    pub fn get_line(&self, line: usize) -> CResult<RopeSlice> {
        Ok(self.text.get_line(line).expect("no line"))
    }

    pub fn get_char(&self, idx: usize) -> CResult<char> {
        Ok(self.text.get_char(idx).expect("no char"))
    }

    pub fn len_lines(&self) -> CResult<usize> {
        Ok(self.text.len_lines())
    }

    pub fn get_cursor_pos(&self) -> CResult<(usize, usize)> {
        let y = self.text.byte_to_line(self.cursor);
        let x = self.text.byte_to_char(self.cursor) - self.text.line_to_byte(y);
        debug!("{:?}", (x, y));
        Ok((x, y))
    }

    pub fn move_cursor_h(&mut self, delta: isize) {
        if delta < 0 {
            if self.cursor != 0 {
                self.cursor -= delta.abs() as usize
            }
        } else {
            if self.cursor != self.text.len_bytes() {
                self.cursor += delta as usize
            }
        }
    }

    pub fn move_cursor_v(&mut self, delta: isize) {
        let line = self.text.byte_to_line(self.cursor);
        let x = self.cursor - self.text.line_to_byte(line);
        if delta < 0 {
            if line != 0 {
                let next = line - delta.abs() as usize;
                let bnext = self.text.line_to_byte(next);

                if self.text.line(next).len_bytes() > x {
                    self.cursor = bnext + x;
                } else {
                    self.cursor = bnext + self.text.line(next).len_bytes();
                }
            }
        } else {
            if line < self.text.len_lines() - 1 {
                let next = line + delta as usize;
                let bnext = self.text.line_to_byte(next);

                if self.text.line(next).len_bytes() > x {
                    self.cursor = bnext + x;
                } else {
                    self.cursor = bnext + self.text.line(next).len_bytes();
                }
            }
        }
    }

    pub fn move_cursor_end(&mut self) {
        let line = self.text.byte_to_line(self.cursor);
        self.cursor = self.text.line_to_byte(line) + self.text.line(line).len_bytes() - 1;
    }

    pub fn move_cursor_home(&mut self) {
        let line = self.text.byte_to_line(self.cursor);
        self.cursor = self.text.line_to_byte(line);
    }
}
