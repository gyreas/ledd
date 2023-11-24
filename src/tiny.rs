use crossterm::{
    cursor::*,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

use std::io::{stdout, Write as _};

pub fn tiny_input() -> crossterm::Result<()> {
    let mut stdout = stdout();

    let mut prompt_buf = String::new();
    let mut count_enters = 0;

    enable_raw_mode()?;
    stdout.execute(Clear(ClearType::FromCursorUp))?;

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
                    stdout.write_all(prompt_buf.as_bytes())?;
                    writeln!(&mut stdout)?;
                    stdout
                        .queue(MoveToNextLine(1))?
                        //exits
                        .execute(Clear(ClearType::FromCursorDown))?;
                    disable_raw_mode()?;
                    std::process::exit(1)
                }

                KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    stdout.queue(Clear(ClearType::FromCursorUp))?;
                    count_enters = 0;
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    stdout.queue(SavePosition)?;
                    if prompt_buf.is_empty() {
                        stdout.queue(MoveTo(0, count_enters))?;
                        writeln!(&mut stdout, "-")?;
                    } else {
                        // for line in &lines {
                        //     stdout.queue(MoveTo(0, enters))?;
                        //     writeln!(&mut stdout, "{}", line)?;
                        // }
                        // }
                        // lines.clear();
                        // }
                        // KeyEvent {
                        //     code: KeyCode::Enter,
                        //     modifiers: KeyModifiers::NONE,
                        //     kind: KeyEventKind::Press,
                        //     state: KeyEventState::NONE,
                        // } => {
                        let edge = size()?;
                        stdout.queue(MoveTo(0, count_enters))?;
                        writeln!(&mut stdout, "{}", prompt_buf)?;
                        count_enters += 1;
                        // lines.push(term_buf.clone());
                        prompt_buf.clear();
                        stdout.queue(RestorePosition)?;

                        stdout
                            .queue(Clear(ClearType::CurrentLine))?
                            .queue(MoveTo(0, edge.1))?;
                    }
                }
                key => match key.code {
                    KeyCode::Backspace => {
                        let _ = prompt_buf.pop();
                        let _ = stdout.queue(MoveLeft(1))?.write(" ".as_bytes())?;
                        stdout.queue(MoveLeft(1))?;
                    }
                    KeyCode::Char(ch) if ch.is_alphanumeric() || ch.is_whitespace() => {
                        prompt_buf.push(ch);
                        stdout.write_all(ch.to_string().as_bytes())?;
                    }
                    _ => (),
                },
            }
        }
        stdout.flush()?;
    }
}
