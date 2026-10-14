use crate::models::cell::Cell;
use crate::utils::neighbour::{NEIGHBOURS, neighbour_coords};
use crate::utils::row_major::{rowcol_to_rowmajor, rowmajor_to_rowcol};
use crate::models::reveal_state::RevealState;
use rand::seq::SliceRandom;

pub struct Board {
    cells: Vec<Cell>, // use row-major order (row * width + col)
    cols: usize,
    rows: usize,
    num_mines: usize,
}

impl Board {
    pub fn new(rows: usize, columns: usize, num_mines: usize) -> Result<Self, &'static str> {
        if num_mines > rows * columns - 1 {
            return Err(
                "There can't be more mines than cells in a board. There must be atleast an empty space to start",
            ); // we use a result to avoid program from crashing and to comunicate the error.
        }

        Ok(Self {
            cells: vec![Cell::default(); rows * columns],
            cols: columns,
            rows,
            num_mines,
        })
    }

    pub fn generate(&mut self, row: usize, col: usize) {
        self.fill_mines(rowcol_to_rowmajor(row, col, self.cols));
        self.fill_numbers();
    }

    pub fn flag_cell(&mut self, row: usize, col: usize) {
        let index = rowcol_to_rowmajor(row, col, self.cols);
        let cell = self.cells[index];

        if !cell.revealed {
            self.cells[index].flagged = !self.cells[index].flagged;
        }
    }

    pub fn reveal_cell(&mut self, row: usize, col: usize) -> RevealState {
        let index = rowcol_to_rowmajor(row, col, self.cols);

        if self.cells[index].revealed || self.cells[index].flagged {
            return RevealState::Failed;
        }

        if self.cells[index].mine {
            return RevealState::Mine;
        }

        self.cells[index].revealed = true;
        
        if self.cells[index].adjacent_mines == 0 {
            let neighbours = self.find_neighbours(index);
            
            for neighbour_coord in neighbours {
                let coords = rowmajor_to_rowcol(neighbour_coord, self.cols);
                self.reveal_cell(coords.0, coords.1);
            }
        }

        RevealState::Empty
    }

    pub fn reveal_all(&mut self) {
        for cell in &mut self.cells {
            cell.revealed = true;
        }
    }

    pub fn all_mines_cleared(&self) -> bool {
        self.cells.iter().all(|cell| cell.revealed || cell.mine)
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    fn fill_mines(&mut self, starting_coord: usize) {
        let starting_coords = {
            let mut coords: Vec<usize> = vec![starting_coord];
            coords.append(&mut self.find_neighbours(starting_coord));
            coords
        };
        
        // I use a block so I can get an immutable reference to the shuffled vector.
        let possible_coords = {
            let mut coords: Vec<usize> = (0..self.cells.len())
                .filter(|&i| !starting_coords.contains(&i)) // we use .filter to exclude the starting coordinate using an anonymous function
                .collect(); // we use .collect to transform the range into a vector of indices
            coords.shuffle(&mut rand::rng());
            coords
        };

        if possible_coords.len() > 0 {
            for i in 0..self.num_mines {
                self.cells[possible_coords[i]].mine = true;
            }
        }
    }

    fn fill_numbers(&mut self) {
        for i in 0..self.cells.len() {
            let mut count = 0;

            for coord in self.find_neighbours(i) {
                if self.cells[coord].mine {
                    count += 1;
                }
            }

            self.cells[i].adjacent_mines = count;
        }
    }

    fn find_neighbours(&self, index: usize) -> Vec<usize> {
        let mut neighbours_coords = vec![];

        for neighbour in NEIGHBOURS {
            if let Some(coords) = neighbour_coords(
                rowmajor_to_rowcol(index, self.cols),
                neighbour,
                self.rows,
                self.cols,
            ) {
                neighbours_coords.push(rowcol_to_rowmajor(coords.0, coords.1, self.cols));
            }
        }

        neighbours_coords
    }
}