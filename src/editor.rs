use crossterm::{event::KeyModifiers, style::Stylize};
use log::debug;
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
    bar: Rect,
    term: Term,
    status_message: StatusMessage,
}

impl Editor {
    pub fn new() -> Self {
        let initial_status = String::from("HELP: Ctrl-Q = quit | Ctrl-S = save");
        let args: Vec<String> = env::args().collect();
        debug!("{:?} {:?}", args, args.len());
        let document = if args.len() > 1 {
            Document::open(&args[1]).unwrap()
        } else {
            Document::new().unwrap()
        };
        let term = Term::new();
        let edit = Rect::new(
            PADDING,
            0,
            term.size().width as usize,
            term.size().height as usize,
        );
        let bar = Rect::new(
            0,
            term.size().height as usize,
            term.size().width as usize,
            term.size().height as usize + 1,
        );
        Editor {
            quit: false,
            cursor: Position::from_rect(&edit),
            offset: Position::default(),
            current_line: Line::new(0, 0, 0),
            document,
            edit,
            bar,
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
                    if self.document.filename.is_none() {
                        self.prompt("filename: ");
                    }
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
        for y in self.edit.tl.y..self.edit.br.y + 1 {
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
        let mut filename = "[No Name]".to_string();
        if let Some(name) = &self.document.filename {
            filename = name.clone();
            filename.truncate(20)
        }
        let line_indicator = format!("{} / {} ", self.current_line.idx + 1, self.document.len());
        let len = filename.len() + line_indicator.len();
        let mut status = String::new();
        if self.bar.br.x > len {
            status.push_str(&" ".repeat(self.bar.br.x - len));
        }
        status = format!("{filename}{status}{line_indicator}");
        status.truncate(self.bar.br.x);

        for y in self.bar.tl.y..self.bar.br.y + 1 {
            if y == self.bar.tl.y {
                self.term.write_line(
                    &Position {
                        x: self.bar.tl.x,
                        y: self.bar.tl.y,
                    },
                    status.clone().black().on_grey().to_string(),
                    self.cursor,
                );
            } else {
                if Instant::now() - self.status_message.time < Duration::new(5, 0) {
                    self.term.write_line(
                        &Position {
                            x: self.bar.tl.x,
                            y: self.bar.tl.y + 1,
                        },
                        self.status_message.text.clone(),
                        self.cursor.clone(),
                    );
                }
            }
        }
    }

    fn prompt(&mut self, prompt: &str) {
        let last_cur_pos = self.cursor.clone();
        let mut result = String::new();
        self.cursor = Position::new(self.bar.tl.x + prompt.len(), self.bar.tl.y + 1);
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh();
            if let Ok(key) = self.term.get_input() {
                match key.code {
                    Key::Left => {
                        if self.cursor.x > prompt.len() {
                            self.cursor.x -= 1;
                        }
                    }
                    Key::Right => {
                        if self.cursor.x < prompt.len() + result.len() {
                            self.cursor.x += 1;
                        }
                    }
                    Key::Char(c) => {
                        result.push(c);
                        self.cursor.x += 1;
                    }
                    Key::Backspace => {
                        result.pop();
                        self.cursor.x -= 1;
                    }
                    Key::Delete => {
                        if self.cursor.x < prompt.len() + result.len() {
                            result.remove(self.cursor.x - prompt.len());
                        }
                    }
                    Key::Enter => {
                        self.status_message.text = String::new();
                        self.cursor = last_cur_pos;
                        self.document.filename = Some(result);
                        break;
                    }
                    _ => (),
                }
            };
        }
    }

    fn exit(&mut self) {
        if self.term.exit().is_ok() {
            exit(0);
        }
    }
}
