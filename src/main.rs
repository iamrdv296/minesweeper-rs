use std::io::{self, Write};
use crate::models::reveal_state::RevealState;
use crate::{display::display_board::display_board, models::board::Board};
mod display;
mod display_config;
mod models;
mod utils;


use crossterm::style::Print;
use crossterm::terminal::Clear;


fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    
    let mut board = Board::new(10, 10, 11)
        .expect("Invalid board configuration");

    let mut first_click = true;
    let mut running = true;

    while running {
        display_board(&mut stdout, &board)?;

        crossterm::execute!(
            stdout,
            Print("\nEnter row and column (e.g. 3 5), or 3 5 f to flag: ")
        )?;

        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut parts = input.split_whitespace();

        let row = match parts.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(n) if (1..=board.get_rows()).contains(&n) => n - 1,
            _ => {
                println!("Invalid row.");
                continue;
            }
        };

        let col = match parts.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(n) if (1..=board.get_cols()).contains(&n) => n - 1,
            _ => {
                println!("Invalid column.");
                continue;
            }
        };

        let flag = matches!(parts.next(), Some("f" | "F"));

        if flag {
            board.flag_cell(row, col);
            continue;
        }

        if first_click {
            board.generate(row, col);
            first_click = false;
        }

        match board.reveal_cell(row, col) {
            RevealState::Mine => {
                board.unflag_all();
                board.reveal_all();
                display_board(&mut stdout, &board)?;

                println!("\n💥 Game over!");
                running = false;
            }

            RevealState::Failed => {
                println!("That cell is already revealed or flagged.");
            }

            RevealState::Empty => {
                if board.all_mines_cleared() {
                    board.reveal_all();
                    display_board(&mut stdout, &board)?;

                    println!("\n🎉 Congratulations! You won!");
                    running = false;
                }
            }
        }
    }

    Ok(())
}
