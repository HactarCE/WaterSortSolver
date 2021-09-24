use std::collections::HashSet;
use std::io::Write;

use crossterm::{cursor, event, terminal, QueueableCommand, Result};
use itertools::Itertools;

use crate::draw;
use crate::puzzle::{Puzzle, COLORS};

pub fn input_puzzle() -> Result<Option<Puzzle>> {
    const OPTION_CHARS: &str = "1234567890QWERTYUIOPASDFGHJKLZXCVBNM";

    let mut options = vec![];
    {
        let mut used_chars = HashSet::new();
        for &color in COLORS {
            for c in (color.simple_name().to_string() + color.name() + OPTION_CHARS).chars() {
                let c = c.to_ascii_uppercase();
                if !used_chars.contains(&c) {
                    used_chars.insert(c);
                    options.push((c, color));
                    break;
                }
            }
        }
    }

    let mut puzzle = Puzzle::new();
    if let Some(serialized) = std::env::args().skip(1).next() {
        puzzle = Puzzle::deser(&serialized).unwrap();
    }

    let mut redraw = true;

    std::io::stdout().queue(cursor::Hide)?;

    loop {
        if redraw {
            std::io::stdout().queue(terminal::Clear(terminal::ClearType::All))?;
            draw::draw_color_palette((1, 1), &options)?;
            draw::draw_puzzle((1, 4), &puzzle)?;
            std::io::stdout().flush()?;
            redraw = false;
        }

        match event::read()? {
            event::Event::Key(event::KeyEvent { code, modifiers: _ }) => match code {
                event::KeyCode::Backspace => {
                    if let Ok(v) = puzzle.last_vial().pop() {
                        *puzzle.last_vial() = v;
                    } else {
                        puzzle.pop_vial();
                    }
                    redraw = true;
                }
                event::KeyCode::Enter => return Ok(Some(puzzle)),
                event::KeyCode::Tab | event::KeyCode::Char(' ') => {
                    puzzle.push_vial();
                    redraw = true;
                }
                event::KeyCode::Char(c) => {
                    for &(ch, color) in &options {
                        if c.eq_ignore_ascii_case(&ch) {
                            if let Ok(v) = puzzle.last_vial().push(color) {
                                *puzzle.last_vial() = v;
                                redraw = true;
                                break;
                            }
                        }
                    }
                }
                event::KeyCode::Esc => return Ok(None),
                _ => (),
            },
            event::Event::Resize(_, _) => redraw = true,
            _ => (),
        }
    }
}
