use crossterm::{
    cursor::{self, *},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, ScrollDown, ScrollUp},
    ExecutableCommand, QueueableCommand,
};

use std::fmt::Write as _;
use std::io::{stdout, Cursor as CursoredBuffer, Write as _};

struct Lino {
    lineno: u16,
    cursor_col: u16,
}

pub fn tiny_input() -> crossterm::Result<()> {
    enable_raw_mode()?;
    let (ncols, nrows) = size()?;
    let mut stdout = stdout();
    let mut term_buf = CursoredBuffer::new(Vec::new());
    let mut cursor_pos = term_buf.position();

    loop {
        if let Event::Key(keyev) = event::read()? {
            match keyev {
                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    write!(&mut stdout, "Wrote: ")?;
                    stdout.write_all(term_buf.get_mut())?;
                    writeln!(&mut stdout)?;
                    stdout
                        .queue(MoveToNextLine(1))?
                        //exits
                        .execute(Clear(ClearType::FromCursorDown))?;
                    disable_raw_mode()?;
                    std::process::exit(1)
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    cursor_pos += 1;
                    term_buf.set_position(cursor_pos);

                    writeln!(&mut term_buf, "Newline")?;
                    writeln!(&mut stdout, "Newline")?;
                    stdout.queue(MoveToNextLine(1))?;
                }
                key => match key.code {
                    KeyCode::Char('j') => {
                        write!(&mut stdout, "size: {}", nrows - 1)?;
                        write!(&mut stdout, "jp: {cursor_pos}")?;
                        if cursor_pos + 1 < nrows as u64 && !term_buf.get_ref().is_empty() {
                            cursor_pos += 1;
                            term_buf.set_position(cursor_pos);
                            write!(&mut stdout, "jc: {cursor_pos}")?;
                            stdout.queue(MoveToNextLine(1))?;
                            //     stdout.queue(ScrollUp(1))?;
                        }
                    }
                    KeyCode::Char('k') => {
                        write!(&mut stdout, "kc: {cursor_pos}")?;
                        if cursor_pos >= 1 {
                            cursor_pos -= if cursor_pos >= 1 { 1 } else { 0 };
                            term_buf.set_position(cursor_pos);

                            // stdout.queue(ScrollDown(1))?;
                            stdout.queue(MoveToPreviousLine(1))?;
                        }
                    }
                    KeyCode::Char('h') => {
                        stdout.queue(MoveLeft(1))?;
                    }
                    KeyCode::Char('l') => {
                        stdout.queue(MoveRight(1))?;
                    }
                    KeyCode::Backspace => {
                        stdout.queue(MoveLeft(1))?;
                    }
                    _ => (),
                },
            }
        }
        stdout.flush()?;
    }
}
