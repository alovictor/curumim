use ropey::{Rope, RopeSlice};
use std::{
    collections::VecDeque,
    fs::{self, File},
    path::PathBuf,
};

use crate::prelude::*;

#[derive(Clone)]
pub enum EditorCmd {
    InsertChar(usize, char),
    DeleteChar(usize, char),
}

pub struct Editor {
    path: PathBuf,
    pub text: Rope,
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

    pub fn insert(&mut self, c: char) -> CResult<()> {
        let idx = self.cursor;
        self.insert_char(idx, c)?;
        self.cmds.push_back(EditorCmd::InsertChar(idx, c));
        Ok(())
    }

    pub fn delete(&mut self) -> CResult<()> {
        let idx = self.cursor;
        let c = self.text.char(idx);
        self.delete_char(idx)?;
        self.cmds.push_back(EditorCmd::DeleteChar(idx, c));
        Ok(())
    }

    pub fn undo(&mut self) -> CResult<()> {
        match self.cmds.pop_back() {
            Some(cmd) => {
                match cmd {
                    EditorCmd::InsertChar(idx, _) => self.delete_char(idx),
                    EditorCmd::DeleteChar(idx, c) => self.insert_char(idx, c)
                }
            },
            None => Ok(())
        }
    }

    fn insert_char(&mut self, idx: usize, c: char) -> CResult<()> {
        self.text.insert_char(idx, c);
        self.move_cursor_h(1);
        Ok(())
    }

    fn delete_char(&mut self, idx: usize) -> CResult<()> {
        if idx != 0 {
            self.text.remove(idx - 1..idx);
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
        match self.text.get_char(idx) {
            Some(c) => Ok(c),
            None => Ok(' ')
        }
    }

    pub fn len_lines(&self) -> CResult<usize> {
        Ok(self.text.len_lines())
    }

    pub fn len_line(self, line: usize) -> CResult<usize> {
        Ok(self.text.line(line).len_chars())
    }

    pub fn get_cursor_pos(&self) -> CResult<usize> {
        Ok(self.cursor)
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
        let line = self.text.char_to_line(self.cursor);
        let x = self.cursor - self.text.line_to_char(line);
        // CIMA
        if delta < 0 {
            if line != 0 {
                let next_line = line - delta.abs() as usize;
                let len = self.text.line(next_line).len_chars();
                let next_char = self.text.line_to_char(next_line);

                if len > x {
                    self.cursor = next_char + x;
                } else {
                    self.cursor = next_char + len - 1;
                }
            }
        // BAIXO
        } else {
            if line < self.text.len_lines() - 1 {
                let next_line = line + delta as usize;
                let len = self.text.line(next_line).len_chars();
                let next_char = self.text.line_to_char(next_line);

                if len > x {
                    self.cursor = next_char + x;
                } else {
                    self.cursor = next_char + len - 1;
                }
            }
        }
    }

    pub fn move_cursor_end(&mut self) {
        let line = self.text.char_to_line(self.cursor);
        self.cursor = self.text.line_to_char(line) + self.text.line(line).len_chars() - 1;
    }

    pub fn move_cursor_home(&mut self) {
        let line = self.text.char_to_line(self.cursor);
        self.cursor = self.text.line_to_char(line);
    }
}
