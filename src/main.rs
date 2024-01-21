#[allow(dead_code)]
pub mod editor;
pub mod prelude;
pub mod terminal;

use std::path::PathBuf;

use clap::Parser;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use log::LevelFilter;
use simple_logging;

use crate::prelude::*;

// Argumentos da linha de comando
#[derive(Parser)]
struct Args {
    #[arg()]
    /// file to edit
    file_path: Option<PathBuf>,
}

fn main() -> CResult<()> {
    // logger
    simple_logging::log_to_file("curumim.log", LevelFilter::Debug).unwrap();

    let args = Args::parse();

    // as duas structs principais do programa. São responsáveis por controlar o terminal e o editor, respectivamente
    let mut term = Terminal::new();
    let mut ed = Editor::new(args.file_path)?;

    // guardamos o buffer anterior para ver se houveram alterações, evitando flushs desnecessários
    // aproveitamentos para inicializar com a tela limpa e renderizamos logo em seguida
    let prev_buf = term.init()?;
    term.render(&prev_buf)?;

    // event loop
    loop {
        let buf = update(&mut ed, &mut term)?;
        term.render(&buf)?;

        match read().unwrap() {
            Event::Key(e) => {
                if e.modifiers == KeyModifiers::CONTROL && e.code == KeyCode::Char('s') {
                    ed.save_file()?
                } else {
                    match e.code {
                        KeyCode::Esc => break,
                        KeyCode::Char(c) => ed.insert_char(c)?,
                        KeyCode::Backspace => ed.delete_char()?,
                        KeyCode::Enter => ed.insert_char('\n')?,
                        KeyCode::Up => ed.move_cursor_v(-1),
                        KeyCode::Down => ed.move_cursor_v(1),
                        KeyCode::Left => ed.move_cursor_h(-1),
                        KeyCode::Right => ed.move_cursor_h(1),
                        KeyCode::End => ed.move_cursor_end(),
                        KeyCode::Home => ed.move_cursor_home(),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    term.exit()?;

    Ok(())
}

fn update(ed: &mut Editor, term: &mut Terminal) -> CResult<CBuffer> {
    let mut buf = Vec::default();
    let size = term.get_size()?;
    let cpos = ed.get_cursor_pos()?;

    for y in 0..size.1 {
        if y < ed.len_lines()? {
            let line = ed.get_line(y)?;
            for x in 0..size.0 {
                if let Some(c) = line.get_char(x) {
                    if y == cpos.1 && x == cpos.0 {
                        if c == '\n' {
                            buf.push(Cell::new(" ".to_string(), Highlight::Cursor))
                        } else {
                            buf.push(Cell::new(c.to_string(), Highlight::Cursor))
                        }
                    } else {
                        buf.push(Cell::new(c.to_string(), Highlight::Text))
                    }
                } else {
                    buf.push(Cell::new(" ".to_string(), Highlight::Text))
                }
            }
        } else {
            for _ in 0..size.0 {
                buf.push(Cell::new(" ".to_string(), Highlight::Text))
            }
        }
    }

    Ok(buf)
}
