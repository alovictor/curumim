use crossterm::{event::KeyModifiers, style::Stylize};
use std::{
    cmp::min,
    env,
    process::exit,
    time::{Duration, Instant},
};

use crate::{Document, Key, Line, Position, Rect, Term};

const PADDING: usize = 4;

struct StatusMessage {
    text: String,
    time: Instant,
}
impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    quit: bool,
    cursor: Position,
    offset: Position,
    current_line: Line,
    document: Document,
    edit: Rect,
    term: Term,
    status_message: StatusMessage,
}

impl Editor {
    pub fn new() -> Self {
        let mut initial_status = String::from("HELP: Ctrl-Q = quit");
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            if let Ok(doc) = Document::open(file_name) {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };
        let term = Term::new();
        let edit = Rect::new(
            PADDING,
            0,
            term.size().width as usize,
            term.size().height as usize - 1,
        );
        Editor {
            quit: false,
            cursor: Position::from_rect(&edit),
            offset: Position::default(),
            current_line: Line::new(0, 0, 0),
            document,
            edit,
            term,
            status_message: StatusMessage::from(initial_status),
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.term.clear().is_ok() {
                self.update();
                self.refresh();
                self.process_keys();
            }
        }
    }

    fn refresh(&mut self) {
        self.term.hide_cur();
        self.term.move_cur(&Position::default()).unwrap();
        if self.quit {
            self.exit();
        } else {
            self.draw_edit();
            self.draw_bar();
            self.term
                .move_cur(&Position {
                    x: self.cursor.x - self.offset.x,
                    y: self.cursor.y - self.offset.y,
                })
                .unwrap();
        }
        self.term.show_cur();
    }

    fn process_keys(&mut self) {
        let key = self.term.get_input().unwrap();
        let pad = PADDING;

        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                Key::Char('q') => self.quit = true,
                Key::Char('s') => {
                    if self.document.save().is_ok() {
                        self.status_message = StatusMessage::from(format!(
                            "Arquivo salvo em: {:?}",
                            self.document.filename.clone().unwrap()
                        ));
                    }
                }
                _ => (),
            },
            KeyModifiers::NONE => match key.code {
                Key::Up => {
                    if self.cursor.y > self.edit.tl.y {
                        self.cursor.y -= 1;
                    }
                    if self.cursor.x > pad + self.document.get_line(self.cursor.y).unwrap().len() {
                        self.cursor.x = pad + self.document.get_line(self.cursor.y).unwrap().len();
                    }
                }
                Key::Down => {
                    if self.cursor.y < self.document.len() - 1 {
                        self.cursor.y += 1;
                    }
                    if self.cursor.x > pad + self.document.get_line(self.cursor.y).unwrap().len() {
                        self.cursor.x = pad + self.document.get_line(self.cursor.y).unwrap().len();
                    }
                }
                Key::Left => {
                    if self.cursor.x > self.edit.tl.x {
                        self.cursor.x -= 1;
                    } else if self.cursor.y > 0 {
                        self.cursor.y -= 1;
                        self.cursor.x = pad + self.document.get_line(self.cursor.y).unwrap().len();
                    }
                }
                Key::Right => {
                    if self.cursor.x < pad + self.current_line.len() {
                        self.cursor.x += 1;
                    } else if self.cursor.y < min(self.edit.br.y, self.document.len() - 1) {
                        self.cursor.y += 1;
                        self.cursor.x = pad;
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
                Key::Char(c) => {
                    self.document
                        .insert(Position::new(self.cursor.x - pad, self.cursor.y), c)
                        .unwrap();
                    self.cursor.x += 1;
                }
                Key::Backspace => {
                    if self.cursor.x > self.edit.tl.x {
                        self.cursor.x -= 1;
                    } else if self.cursor.y > 0 {
                        self.cursor.y -= 1;
                        self.cursor.x = pad + self.document.get_line(self.cursor.y).unwrap().len();
                    }
                    self.document
                        .delete(Position::new(self.cursor.x - pad, self.cursor.y))
                        .unwrap();
                }
                Key::Delete => {
                    self.document
                        .delete(Position::new(self.cursor.x - pad, self.cursor.y))
                        .unwrap();
                }
                Key::Enter => {
                    self.document
                        .insert(Position::new(self.cursor.x - pad, self.cursor.y), '\n')
                        .unwrap();
                    self.cursor.x = 0 + pad;
                    self.cursor.y += 1;
                }
                _ => (),
            },
            _ => (),
        };
        self.scroll();
    }

    fn update(&mut self) {
        if let Some(line) = self.document.get_line(self.cursor.y) {
            self.current_line = line
        }

        // self.term.set_title("Aloalo".to_string());

        // if !self.cursor.in_range(&self.edit) {
        //     self.cursor = Position::from_rect(&self.edit);
        // }
    }

    fn scroll(&mut self) {
        if self.cursor.y < self.offset.y {
            self.offset.y = self.cursor.y;
        } else if self.cursor.y >= self.offset.y + self.edit.br.y {
            self.offset.y = self.cursor.y - self.edit.br.y + 1;
        }
        if self.cursor.x < self.offset.x {
            self.offset.x = self.cursor.x;
        } else if self.cursor.x >= self.offset.x + self.edit.br.x {
            self.offset.x = self.cursor.x - self.edit.br.x + 1;
        }
    }

    fn draw_edit(&mut self) {
        for y in 0..self.edit.br.y + 1 {
            if let Some(line) = self.document.get_line(y + self.offset.y) {
                let mut string;
                if line.idx < 9 {
                    string = format!("  {}", line.idx + 1)
                } else if line.idx < 99 {
                    string = format!(" {}", line.idx + 1)
                } else {
                    string = format!("{}", line.idx + 1)
                };
                string = format!("{} {}", string, self.document.get_str_line(line.idx));
                self.draw_row(&Position { x: 0, y }, string);
            }
        }
    }

    fn draw_row(&mut self, pos: &Position, string: String) {
        self.term.write_line(pos, string, self.cursor.clone());
    }

    fn draw_bar(&mut self) {
        let mut status;
        let mut filename = "[No Name]".to_string();
        if let Some(name) = &self.document.filename {
            filename = name.clone();
            filename.truncate(20);
        }
        status = filename.to_string();
        let line_indicator = format!("{} / {} ", self.current_line.idx + 1, self.document.len());
        let len = status.len() + line_indicator.len();
        if self.term.size().width as usize > len {
            status.push_str(&" ".repeat(self.term.size().width as usize - len));
        }
        status = format!("{status}{line_indicator}");
        status.truncate(self.term.size().width as usize);

        self.term.write_line(
            &Position {
                x: 0,
                y: self.term.size().height as usize,
            },
            status.black().on_grey().to_string(),
            self.cursor,
        );
        if Instant::now() - self.status_message.time < Duration::new(5, 0) {
            self.term.write_line(
                &Position {
                    x: 0,
                    y: self.term.size().height as usize + 1,
                },
                self.status_message.text.clone(),
                self.cursor.clone(),
            );
        }
    }

    fn exit(&mut self) {
        if self.term.exit().is_ok() {
            exit(0);
        }
    }
}
