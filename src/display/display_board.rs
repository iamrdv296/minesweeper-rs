use std::io::{self, Stdout, Write};

use crate::{
    display::display_cell::display_cell,
    models::{board::Board, cell::{Cell, CellType, get_cell_type}},
    utils::{
        display::add_zeros,
        row_major::rowcol_to_rowmajor,
    },
};
use crate::display_config::*;
use crossterm::style::{Color, Print, SetForegroundColor};

pub fn display_board(stdout: &mut Stdout, board: &Board) -> io::Result<()> {
    let cells = board.get_cells();
    let rows = board.get_rows();
    let cols = board.get_cols();

    let row_num_width = rows.to_string().len();
    let col_num_width = cols.to_string().len();

    crossterm::execute!(stdout, Clear(crossterm::terminal::ClearType::All))?;
    
    display_col_numbers(
        stdout,
        cols,
        &" ".repeat(ROW_MARGIN + row_num_width),
        SPACING,
        col_num_width,
    )?;
    crossterm::execute!(stdout, Print("\r\n".repeat(COL_MARGIN + 1)))?;

    for row in 0..rows {
        let row_cells =
            &cells[rowcol_to_rowmajor(row, 0, cols)..rowcol_to_rowmajor(row, cols, cols)];

        display_row_number(stdout, row + 1, row_num_width)?;
        display_row(
            stdout,
            row_cells,
            &" ".repeat(ROW_MARGIN),
            SPACING,
            col_num_width,
        )?;

        crossterm::execute!(stdout, Print("\r\n"))?
    }

    Ok(())
}

fn display_col_numbers(
    stdout: &mut Stdout,
    cols: usize,
    margin: &str,
    spacing: &str,
    col_num_width: usize,
) -> io::Result<()> {
    crossterm::queue!(stdout, Print(format!("{margin}{spacing}")))?;

    for col in 0..cols {
        let col = col + 1;
        let col = add_zeros(col, col_num_width); // keeps number_width fixed, for example, instead of 2 and 10, we have 02 and 10

        crossterm::queue!(stdout, Print(format!("{col}{spacing}")))?;
    }

    stdout.flush()
}

fn display_row(
    stdout: &mut Stdout,
    row_cells: &[Cell],
    margin: &str,
    spacing: &str,
    cell_width: usize,
) -> io::Result<()> {
    crossterm::queue!(stdout, Print(format!("{margin}{spacing}")))?;

    for cell in row_cells {
        let display = {
            let mut display = display_cell(&cell, FLAG, MINE, OCCULT);

            if let Ok(cell_num) = display.parse::<usize>() {
                if cell_num != 0 {
                    display = add_zeros(cell_num, cell_width);
                } else {
                    display = " ".repeat(cell_width);
                }
            } else {
                display = display.repeat(cell_width);
            }
            display
        };

        let color = get_type_color(get_cell_type(cell));

        crossterm::queue!(
            stdout,
            SetForegroundColor(color),
            Print(format!("{display}")),
            SetForegroundColor(Color::Reset),
            Print(format!("{spacing}"))
        )?;
    }

    stdout.flush()
}

fn display_row_number(stdout: &mut Stdout, row_num: usize, row_num_width: usize) -> io::Result<()> {
    let row_num = add_zeros(row_num, row_num_width);
    crossterm::execute!(stdout, Print(format!("{row_num}")))
}

fn get_type_color(cell_type: CellType) -> Color {
    match cell_type {
        CellType::Flag => Color::Green,
        CellType::Mine => Color::Red,
        CellType::Occult => Color::White,
        CellType::Number => Color::Blue,
    }
}
