use crate::{Input, Position};
use crossterm::{
    cursor,
    event::{read, Event},
    execute, terminal,
};
use std::io::{stdout, Error, ErrorKind, Stdout};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Term {
    size: Size,
    out: Stdout,
}

impl Term {
    #[must_use]
    pub fn new(pad: usize) -> Self {
        let mut out = stdout();
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
            out,
        }
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        execute!(self.out, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn clear_current_line(&mut self) -> Result<(), Error> {
        execute!(self.out, terminal::Clear(terminal::ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cur(&mut self, pos: &Position) -> Result<(), Error> {
        execute!(self.out, cursor::MoveTo(pos.x as u16, pos.y as u16))?;
        Ok(())
    }

    pub fn hide_cur(&mut self) {
        execute!(self.out, cursor::Hide).unwrap();
    }

    pub fn show_cur(&mut self) {
        execute!(self.out, cursor::Show).unwrap();
    }

    pub fn write_line(&mut self, pos: &Position, string: String, curr_pos: Position) {
        if self.move_cur(&Position { x: pos.x, y: pos.y }).is_ok()
            && self.clear_current_line().is_ok()
        {
            print!("{string}");
        };
        self.move_cur(&curr_pos).unwrap();
    }

    pub fn get_input(&mut self) -> Result<Input, Error> {
        if let Ok(Event::Key(e)) = read() {
            return Ok(Input::from(e));
        }
        Err(Error::new(ErrorKind::Other, "foda meu"))
    }

    pub fn set_title(&mut self, string: String) -> Result<(), Error> {
        execute!(self.out, terminal::SetTitle(string))?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), Error> {
        if self.clear().is_ok() {
            self.show_cur();
            execute!(self.out, terminal::LeaveAlternateScreen).unwrap();
            // self.out.flush().unwrap();
        };
        Ok(())
    }
}
