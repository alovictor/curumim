use crossterm::{
    cursor::{Hide, MoveTo},
    queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use std::io::{stdout, Stdout, Write};

use crossterm::style::Color;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Highlight {
    Text,
    Cursor,
}

impl Highlight {
    /// returns the foreground color
    pub fn get_foreground(self) -> Color {
        match self {
            Self::Text => Color::White,
            Self::Cursor => Color::Black,
        }
    }

    /// returns the background color
    pub fn get_background(self) -> Color {
        match self {
            Self::Text => Color::Black,
            Self::Cursor => Color::White,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub symbol: String,
    pub style: Highlight,
}

impl Cell {
    pub fn new(s: String, hl: Highlight) -> Self {
        Self {
            symbol: s,
            style: hl,
        }
    }
}

pub struct Terminal {
    size: (usize, usize),
    out: Stdout,
}

impl Terminal {
    pub fn new() -> CResult<Self> {
        let u16_size = terminal::size()?;
        let size = (u16_size.0 as usize, u16_size.1 as usize);
        Ok(Self {
            size,
            out: stdout(),
        })
    }

    pub fn init(&mut self) -> CResult<CBuffer> {
        queue!(stdout(), EnterAlternateScreen)?;
        queue!(stdout(), Clear(ClearType::All))?;
        enable_raw_mode()?;
        stdout().flush()?;
        Ok(self.create_buffer())
    }

    pub fn create_buffer(&self) -> CBuffer {
        let mut buf = Vec::default();
        for _ in 0..self.size.0 * self.size.1 {
            buf.push(Cell::new(" ".to_string(), Highlight::Text))
        }
        buf
    }

    pub fn render(&mut self, buf: &CBuffer) -> CResult<()> {
        queue!(stdout(), Hide)?;
        queue!(stdout(), Clear(ClearType::All))?;

        for index in 0..buf.len() {
            self.print_cell(&buf[index], index)?
        }

        stdout().flush()?;
        Ok(())
    }

    fn print_cell(&mut self, c: &Cell, index: usize) -> CResult<()> {
        let x = index % self.size.0;
        let y = index / self.size.0;
        queue!(self.out, MoveTo(x as u16, y as u16))?;

        let fg = c.style.get_foreground();
        let bg = c.style.get_background();

        queue!(self.out, SetForegroundColor(fg), SetBackgroundColor(bg))?;
        queue!(self.out, Print(c.symbol.to_owned()))?;
        Ok(())
    }

    pub fn exit(&mut self) -> CResult<()> {
        queue!(stdout(), Clear(ClearType::All))?;
        disable_raw_mode()?;
        queue!(stdout(), LeaveAlternateScreen)?;
        stdout().flush()?;
        std::process::exit(0);
    }

    pub fn get_size(&mut self) -> CResult<&(usize, usize)> {
        Ok(&self.size)
    }
}
