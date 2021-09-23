use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, Result};

mod draw;
mod puzzle;
mod solver;
mod ui;

fn main() -> Result<()> {
    let result = app();
    std::io::stdout().execute(cursor::Show)?;
    result
}

fn app() -> Result<()> {
    std::io::stdout().queue(cursor::Hide)?;

    let p = ui::input_puzzle()?;
    std::io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::MoveTo(0, 0))?
        .execute(cursor::Show)?;

    if let Some(p) = p {
        println!("now solving this puzzle:");
        println!("{}", p.ser());

        let moves = solver::solve(p).unwrap();
        println!("solved!");
        for m in moves {
            println!("{}", m);
        }
    } else {
        println!("canceled");
    }

    Ok(())
}
