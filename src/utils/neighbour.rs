// must use isize here because of negative offsets
// also, for fixed-size arrays, the type must be known at compile time as [type; size]
pub const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, 1), (0, 1), (1, 1),
    (-1, 0), (1, 0),
    (-1, -1), (0, -1), (1, -1),
];

pub fn neighbour_coords(coords: (usize, usize), offset: (isize, isize), rows: usize, cols: usize) -> Option<(usize, usize)> {
    let n_row = coords.0 as isize + offset.0;
    let n_col = coords.1 as isize + offset.1;

    if n_row >= 0 && n_row < rows as isize && n_col >= 0 && n_col < cols as isize {
        Some((n_row as usize, n_col as usize))
    } else {
        None
    }
}
