pub fn rowcol_to_rowmajor(row: usize, col: usize, cols: usize) -> usize {
    (row * cols) + col
}

pub fn rowmajor_to_rowcol(rowmajor: usize, cols: usize) -> (usize, usize) {
    (rowmajor / cols, rowmajor % cols)
}