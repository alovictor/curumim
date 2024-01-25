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
    simple_logging::log_to_file("curumim.log", LevelFilter::Debug)?;

    let args = Args::parse();

    // as duas structs principais do programa. São responsáveis por controlar o terminal e o editor, respectivamente
    let mut term = Terminal::new()?;
    let mut ed = Editor::new(args.file_path)?;

    // guardamos o buffer anterior para ver se houveram alterações, evitando flushs desnecessários
    // aproveitamentos para inicializar com a tela limpa e renderizamos logo em seguida
    let mut prev_buf = term.init()?;

    // event loop
    loop {
        update_and_render(&mut prev_buf, &mut ed, &mut term)?;
        
        match read().unwrap() {
            Event::Key(e) => {
                if e.modifiers == KeyModifiers::CONTROL && e.code == KeyCode::Char('s') {
                    ed.save_file()?
                } else if e.modifiers == KeyModifiers::CONTROL && e.code == KeyCode::Char('z') {
                    ed.undo()?;
                } else {
                    match e.code {
                        KeyCode::Esc => break,
                        KeyCode::Char(c) => ed.insert(c)?,
                        KeyCode::Backspace => ed.delete()?,
                        KeyCode::Enter => ed.insert('\n')?,
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

fn update_and_render(prev_buf: &mut CBuffer, ed: &mut Editor, term: &mut Terminal) -> CResult<()> {
    let mut buf = term.create_buffer();
    let size = term.get_size()?;
    let cpos = ed.get_cursor_pos()?;
    let chars = ed.text.chars();
    let mut line = 0;

    for idx in 0..chars.len() + 1 {
        let c = ed.get_char(idx)?;
        let x = idx - ed.text.line_to_char(line);
        let buf_id = x + line * size.0;

        if c == '\n' {
            line += 1;
        } else {
            buf[buf_id] = Cell::new(c.to_string(), Highlight::Text);
        }

        if idx == cpos {
            buf[buf_id].style = Highlight::Cursor;
        }
    }

    if check_buffers(&buf, &prev_buf) {
        term.render(&buf)?;
    }

    Ok(())
}

// se os buffers forem diferentes retorna true
// se os buffers forem iguais retorna false
fn check_buffers(buf1: &CBuffer, buf2: &CBuffer) -> bool {
    if buf1.len() == buf2.len() {
        for i in 0..buf1.len() {
            if buf1[i] != buf2[i] {return true}
        }
    }
    false
}
