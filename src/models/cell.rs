// needs default to be able to be created with default values. 
// needs clone to be able to vec! in board
#[derive(Debug, Default, Clone, Copy)] 
pub struct Cell {
    pub mine: bool,
    pub revealed: bool,
    pub adjacent_mines: usize,
    pub flagged: bool,
}

pub enum CellType {
    Mine,
    Flag, 
    Number,
    Occult
}

pub fn get_cell_type(cell: &Cell) -> CellType {
    if cell.flagged {
        CellType::Flag
    } else if !cell.revealed {
        CellType::Occult
    }  else if cell.mine {
        CellType::Mine
    } else {
        CellType::Number
    }
}
