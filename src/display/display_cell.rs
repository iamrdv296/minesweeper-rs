use crate::models::cell::{Cell, get_cell_type, CellType};

pub fn display_cell(cell: &Cell, flagged: &str, mine: &str, occult: &str) -> String {
    let cell_type = get_cell_type(cell);

    match cell_type {
        CellType::Flag => flagged.to_string(),
        CellType::Mine => mine.to_string(),
        CellType::Occult => occult.to_string(),
        CellType::Number => cell.adjacent_mines.to_string()
    }
}