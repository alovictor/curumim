use crossterm::{event::KeyModifiers, style::Stylize};
use std::{cmp::min, env, io::stdout, process::exit};

use crate::{Document, Key, Line, Position, Rect, Term};

// const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    quit: bool,
    cursor: Position,
    offset: Position,
    current_line: Line,
    document: Document,
    edit: Rect,
    term: Term,
}

impl Editor {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };
        let term = Term::new(stdout());
        let edit = Rect::new(
            4,
            0,
            term.size().width as usize,
            term.size().height as usize,
        );
        Editor {
            quit: false,
            cursor: Position::from_rect(&edit),
            offset: Position::default(),
            current_line: Line::new(0, 0, 0),
            document,
            edit,
            term,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.term.clear();
            self.refresh();
            self.process_keys();
            self.update();
        }
    }

    fn refresh(&mut self) {
        self.term.hide_cur();
        self.term.move_cur(&Position::default());
        if self.quit {
            self.exit();
        } else {
            self.draw_edit();
            self.draw_bar();
            self.scroll();
            self.term.move_cur(&self.cursor);
        }
        self.term.show_cur();
    }

    fn process_keys(&mut self) {
        let key = self.term.get_input().unwrap();
        let pad = self.edit.tl.x.clone();

        match key.modifiers {
            KeyModifiers::CONTROL => {
                if key.code == Key::Char('q') {
                    self.quit = true;
                }
            }
            KeyModifiers::NONE => match key.code {
                Key::Up => {
                    if self.cursor.y > self.edit.tl.y {
                        self.cursor.y -= 1;
                    }
                    if self.cursor.x > pad + self.document.get_line(self.cursor.y).unwrap().len() {
                        self.cursor.x = pad + self.document.get_line(self.cursor.y).unwrap().len()
                    }
                }
                Key::Down => {
                    if self.cursor.y < self.edit.br.y - 1 {
                        self.cursor.y += 1
                    }
                    if self.cursor.x > pad + self.document.get_line(self.cursor.y).unwrap().len() {
                        self.cursor.x = pad + self.document.get_line(self.cursor.y).unwrap().len()
                    }
                }
                Key::Left => {
                    if self.cursor.x > self.edit.tl.x {
                        self.cursor.x -= 1;
                    } else if self.cursor.y > 0 {
                        self.cursor.y -= 1;
                        self.cursor.x = pad + self.document.get_line(self.cursor.y).unwrap().len()
                    }
                }
                Key::Right => {
                    if self.cursor.x < pad + self.current_line.len() {
                        self.cursor.x += 1;
                    } else if self.cursor.y < self.edit.br.y {
                        self.cursor.y += 1;
                        self.cursor.x = pad
                    }
                }
                Key::Home => self.cursor.x = self.edit.tl.x,
                Key::End => self.cursor.x = pad + self.current_line.len(),
                Key::PageUp => {
                    self.cursor.y = self.edit.tl.y;
                    self.cursor.x = self.edit.tl.x;
                }
                Key::PageDown => {
                    self.cursor.y = min(self.edit.br.y, self.document.len() - 1);
                    self.cursor.x = self.edit.tl.x;
                }
                // Key::Char(c) => self.document.insert_char(c, &self.cursor).unwrap(),
                _ => (),
            },
            _ => (),
        }
    }

    fn update(&mut self) {
        self.current_line = self.document.get_line(self.cursor.y).unwrap();

        if !self.cursor.in_range(&self.edit) {
            self.cursor = Position::from_rect(&self.edit)
        }
    }

    fn scroll(&mut self) {
        if self.cursor.y == self.edit.br.y {
            self.offset.y = self.cursor.y - self.edit.br.y
        }
        // else if y >= offset.y.saturating_add(height) {
        //     offset.y = y.saturating_sub(height).saturating_add(1);
        // }
    }

    fn draw_edit(&mut self) {
        for line in self.offset.y..self.edit.br.y + 1 {
            if let Some(string) = self.document.get_str_line(line) {
                self.draw_row(
                    &Position {
                        x: self.edit.tl.x,
                        y: line,
                    },
                    string,
                );
            }
        }
    }

    fn draw_row(&mut self, pos: &Position, string: String) {
        self.term.write_line(pos, string, self.cursor.clone())
    }

    fn draw_bar(&mut self) {
        let mut status;
        let mut filename = "[No Name]".to_string();
        if let Some(name) = &self.document.filename {
            filename = name.clone();
            filename.truncate(20);
        }
        status = format!("{}", filename);
        let line_indicator = format!(
            "{:?} / {:?} / {} / {}",
            self.offset,
            self.cursor,
            self.edit.br.y,
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        if self.term.size().width as usize > len {
            status.push_str(&" ".repeat(self.term.size().width as usize - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(self.term.size().width as usize);

        self.term.write_line(
            &Position {
                x: 0,
                y: self.term.size().height as usize,
            },
            status.black().on_grey().to_string(),
            self.cursor.clone(),
        )
    }

    fn exit(&mut self) {
        if let Ok(_) = self.term.exit() {
            exit(0);
        }
    }
}
