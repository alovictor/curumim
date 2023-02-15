use crate::{Input, Position};
use crossterm::{
    cursor,
    event::{read, Event},
    execute, terminal,
};
use std::io::{Error, ErrorKind, Stdout};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Term {
    size: Size,
    _out: Stdout,
}

impl Term {
    pub fn new(mut out: Stdout) -> Self {
        execute!(out, terminal::EnterAlternateScreen).unwrap();
        if !terminal::is_raw_mode_enabled().unwrap() {
            terminal::enable_raw_mode().unwrap();
        }
        let size = terminal::size().unwrap();
        Term {
            size: Size {
                width: size.0,
                height: size.1 - 2,
            },
            _out: out,
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear(&mut self) {
        execute!(self._out, terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn move_cur(&mut self, pos: &Position) {
        execute!(self._out, cursor::MoveTo(pos.x as u16, pos.y as u16)).unwrap();
    }

    pub fn hide_cur(&mut self) {
        execute!(self._out, cursor::Hide).unwrap();
    }

    pub fn show_cur(&mut self) {
        execute!(self._out, cursor::Show).unwrap();
    }

    pub fn write_line(&mut self, pos: &Position, string: String, curr_pos: Position) {
        self.move_cur(&Position { x: 0, y: pos.y });
        print!("{string:#}");
        self.move_cur(&curr_pos);
    }

    pub fn get_input(&mut self) -> Result<Input, Error> {
        if let Ok(Event::Key(e)) = read() {
            return Ok(Input::from(e));
        }
        Err(Error::new(ErrorKind::Other, "foda meu"))
    }

    pub fn exit(&mut self) -> Result<(), Error> {
        self.clear();
        self.show_cur();
        execute!(self._out, terminal::LeaveAlternateScreen).unwrap();
        // self._out.flush().unwrap();
        Ok(())
    }
}
