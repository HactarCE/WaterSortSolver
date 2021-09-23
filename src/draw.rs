use crate::puzzle::{Color, Puzzle, Vial};
use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand, Result,
};

pub fn draw_color_palette((x, y): (u16, u16), options: &[(char, Color)]) -> Result<()> {
    const WIDTH: u16 = 3;
    const DX: u16 = 4;

    let mut stdout = std::io::stdout();

    stdout.queue(cursor::MoveTo(x, y))?;
    for &(_ch, color) in options {
        draw_color(color, WIDTH)?;
        stdout.queue(cursor::MoveRight(DX - WIDTH))?;
    }
    stdout.queue(cursor::MoveTo(x + WIDTH / 2, y + 1))?;
    for &(ch, _color) in options {
        stdout
            .queue(style::Print(ch))?
            .queue(cursor::MoveRight(DX - 1))?;
    }

    Ok(())
}

pub fn draw_puzzle((mut x, mut y): (u16, u16), puzzle: &Puzzle) -> Result<()> {
    // let (x, y) = cursor::position().expect("error getting cursor position");

    const DX: u16 = 7;
    const DY: u16 = 6;

    let end_of_top_row = puzzle.0.len() - (puzzle.0.len() / 2);

    let orig_x = x;
    for &vial in &puzzle.0[..end_of_top_row] {
        draw_vial((x, y), vial)?;
        x += DX;
    }
    x = orig_x;
    y += DY;
    for &vial in &puzzle.0[end_of_top_row..] {
        draw_vial((x, y), vial)?;
        x += DX;
    }

    Ok(())
}

fn draw_vial((x, y): (u16, u16), vial: Vial) -> Result<()> {
    let mut stdout = std::io::stdout();

    const WIDTH: u16 = 3;

    stdout
        .queue(cursor::MoveTo(x, y))?
        .queue(style::Print("▗   ▖"))?;
    for row in 1..=4 {
        stdout
            .queue(cursor::MoveTo(x, y + row as u16))?
            .queue(style::Print('▐'))?;
        if let Some(color) = vial.0[4 - row] {
            draw_color(color, WIDTH)?;
        } else {
            stdout.queue(cursor::MoveRight(WIDTH))?;
        }
        stdout.queue(style::Print('▌'))?;
    }
    stdout
        // .queue(cursor::MoveTo(x, y + 5))?
        // .queue(style::Print("▝▀▀▘"))?;
        .queue(cursor::MoveTo(x + 1, y + 5))?
        .queue(style::Print("▔▔▔"))?;
    Ok(())
}

fn draw_color(color: Color, width: u16) -> Result<()> {
    let mut stdout = std::io::stdout();

    let s: String = std::iter::repeat('█').take(width as usize).collect();
    let (r, g, b) = color.rgb();
    stdout.queue(style::PrintStyledContent(
        s.with(crossterm::style::Color::Rgb { r, g, b }),
    ))?;
    Ok(())
}
