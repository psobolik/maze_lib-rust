use crate::maze_generator::cell::Cell;
use crate::maze_generator::coordinates::Coordinates;

/// Represents a maze as a two-dimensional vector of Cells.
#[derive(Debug)]
pub struct CellGrid {
    columns: u32,
    rows: u32,
    cells: Vec<Option<Cell>>,
}

/// Associated functions to create and use a CellGrid struct.
impl CellGrid {
    /// Creates a new, unpopulated CellGrid struct with the given dimensions.
    pub fn new(columns: u32, rows: u32) -> CellGrid {
        let cells = (0..columns * rows).map(|_i| None).collect();
        CellGrid {
            columns,
            rows,
            cells,
        }
    }

    /// Returns the number of columns in the CellGrid struct.
    pub fn columns(&self) -> u32 {
        self.columns
    }

    /// Returns the number of rows in the CellGrid struct.
    pub fn rows(&self) -> u32 {
        self.rows
    }

    /// Returns the given maze coordinates translated into an index into the CellGrid struct.
    /// (private)
    fn get_index(&self, coordinates: &Coordinates) -> usize {
        if self.in_bounds(coordinates) {
            (coordinates.row() * (self.columns as i32) + coordinates.column()) as usize
        } else {
            panic!("Coordinates ({coordinates:?} out of bounds");
        }
    }

    /// Returns the value of the CellGrid struct's cell at the given coordinates.
    pub fn get_cell(&self, coordinates: &Coordinates) -> Option<Cell> {
        let index = self.get_index(coordinates);
        self.cells[index]
    }

    /// Sets the value of the CellGrid struct's cell at the given coordinates.
    pub fn set_cell(&mut self, cell: Cell) {
        let index = self.get_index(&cell.coordinates());
        self.cells[index] = Some(cell);
    }

    /// Returns true if the given coordinates represent a location within the CellGrid struct's bounds.
    pub fn in_bounds(&self, coordinates: &Coordinates) -> bool {
        (0..self.rows).contains(&(coordinates.row() as u32))
            && (0..self.columns).contains(&(coordinates.column() as u32))
    }
}

impl IntoIterator for &CellGrid {
    type Item = Option<Cell>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze_generator::cell_edge::CellEdge;
    use crate::maze_generator::direction::Direction;

    #[test]
    fn columns() {
        let rows = 20;
        let columns = 10;
        let cell_grid = CellGrid::new(columns, rows);
        assert_eq!(columns, cell_grid.columns());
    }

    #[test]
    fn rows() {
        let rows = 20;
        let columns = 10;
        let cell_grid = CellGrid::new(columns, rows);
        assert_eq!(rows, cell_grid.rows());
    }

    #[test]
    fn in_bounds() {
        let rows = 20;
        let columns = 40;
        let cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(columns as i32 - 2, rows as i32 - 2);
        assert!(
            cell_grid.in_bounds(&coordinates),
            "Not in bounds: {coordinates:?}"
        );
    }

    #[test]
    fn row_not_in_bounds() {
        let rows = 20;
        let columns = 20;
        let cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(columns as i32, rows as i32 - 2);
        assert!(!cell_grid.in_bounds(&coordinates));
    }

    #[test]
    fn column_not_in_bounds() {
        let rows = 20;
        let columns = 40;
        let cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(columns as i32, rows as i32 - 2);
        assert!(!cell_grid.in_bounds(&coordinates));
    }

    #[test]
    fn index_first_row_first_column() {
        let rows = 20;
        let columns = 40;
        let mut cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(0, 0);
        let cell = Cell::new(coordinates);
        cell_grid.set_cell(cell);
        index_test_helper(cell_grid, cell);
    }

    #[test]
    fn index_last_row_last_column() {
        let rows = 20;
        let columns = 40;
        let mut cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new((columns - 1) as i32, (rows - 1) as i32);
        let cell = Cell::new(coordinates);
        cell_grid.set_cell(cell);
        index_test_helper(cell_grid, cell);
    }

    #[test]
    fn index_middle_row_middle_column() {
        let rows = 20;
        let columns = 40;
        let row = rows / 2;
        let column = columns / 2;
        let mut cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(column as i32, row as i32);
        let cell = Cell::new(coordinates);
        cell_grid.set_cell(cell);
        index_test_helper(cell_grid, cell);
    }

    fn index_test_helper(cell_grid: CellGrid, cell: Cell) {
        let index = cell_grid.get_index(&cell.coordinates());
        if let Some(cell2) = cell_grid.cells[index] {
            assert_eq!(cell, cell2);
        } else {
            panic!("Problem getting or getting cell")
        }
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn index_row_not_in_bounds_low() {
        let rows = 20;
        let columns = 40;
        let row = -2;
        let column = columns / 2;
        let cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(column as i32, row);
        cell_grid.get_index(&coordinates);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn index_row_not_in_bounds_high() {
        let rows = 20;
        let columns = 40;
        let row = rows;
        let column = columns / 2;
        let cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(column as i32, row as i32);
        cell_grid.get_index(&coordinates);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn index_column_not_in_bounds_low() {
        let rows = 20;
        let columns = 40;
        let row = rows;
        let column = -2;
        let cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(column, row as i32);
        cell_grid.get_index(&coordinates);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn index_column_not_in_bounds_high() {
        let rows = 20;
        let columns = 40;
        let row = rows;
        let column = columns;
        let cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(column as i32, row as i32);
        cell_grid.get_index(&coordinates);
    }

    #[test]
    fn set_and_get_cell() {
        let rows = 10;
        let columns = 8;
        let row = 0;
        let column = 0;
        let mut cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(column, row);
        let mut expected_cell = Cell::new(Coordinates::new(column, row));
        expected_cell.set_edge(&Direction::North, Some(CellEdge::Border));
        expected_cell.set_edge(&Direction::West, Some(CellEdge::Passage));
        cell_grid.set_cell(expected_cell);
        let got_cell = cell_grid.get_cell(&coordinates);
        assert_eq!(Some(expected_cell), got_cell);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn set_cell_not_in_bounds() {
        let rows = 2;
        let columns = 4;
        let row = rows;
        let column = 2;
        let mut cell_grid = CellGrid::new(columns, rows);
        cell_grid.set_cell(Cell::new(Coordinates::new(column, row as i32)));
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn get_cell_not_in_bounds() {
        let rows = 10;
        let columns = 8;
        let row = 0;
        let column = 30;
        let cell_grid = CellGrid::new(columns, rows);
        let coordinates = Coordinates::new(column, row);
        cell_grid.get_cell(&coordinates);
    }
}
