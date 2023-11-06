use rand::Rng;

use crate::maze::cell::Cell;
use crate::maze::cell_edge::CellEdge;
use crate::maze::coordinates::Coordinates;
use crate::maze::direction::Direction;
use crate::maze::Maze;

/// Returns a [Maze] of a given size, fully populated with fully assigned [Cell]s and
/// representing a solvable maze.
pub fn generate(columns: u32, rows: u32) -> Maze {
    let mut maze_generator = MazeGenerator::new(columns, rows);
    maze_generator.populate();
    maze_generator.maze
}

struct MazeGenerator {
    maze: Maze,
}

impl MazeGenerator {
    pub fn new(columns: u32, rows: u32) -> MazeGenerator {
        let cell_grid = Maze::new(columns, rows);
        MazeGenerator { maze: cell_grid }
    }

    pub fn populate(&mut self) {
        let mut visit_stack: Vec<Coordinates> = Vec::new();
        self.add_first_cell(&mut visit_stack);
        while !visit_stack.is_empty() {
            self.process_active_cell(&mut visit_stack);
        }
    }

    fn process_active_cell(&mut self, visit_stack: &mut Vec<Coordinates>) {
        if let Some(cell_coordinates) = visit_stack.last() {
            if let Some(mut current_cell) = self.maze.cell(cell_coordinates) {
                if current_cell.is_fully_assigned() {
                    visit_stack.pop();
                    return;
                }
                let direction = current_cell.random_unassigned_direction();
                let neighbor_coordinates = current_cell.coordinates() + direction.coordinates();
                if self.maze.in_bounds(&neighbor_coordinates) {
                    match self.maze.cell(&neighbor_coordinates) {
                        Some(mut neighbor_cell) => {
                            self.create_wall(&mut current_cell, &mut neighbor_cell, &direction)
                        }
                        None => {
                            // The neighbor hasn't been visited, so create cell there with a passage
                            let mut neighbor_cell =
                                self.add_new_cell(visit_stack, neighbor_coordinates);
                            self.create_passage(&mut current_cell, &mut neighbor_cell, &direction);
                        }
                    }
                } else {
                    // Neighbor is out of bounds, so create a border in the selected direction
                    self.create_border(&mut current_cell, &direction);
                }
            }
        }
    }

    fn add_first_cell(&mut self, visit_stack: &mut Vec<Coordinates>) {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..self.maze.rows());
        let column = rng.gen_range(0..self.maze.columns());
        let coordinates = Coordinates::new(column as i32, row as i32);
        self.add_new_cell(visit_stack, coordinates);
    }

    fn add_new_cell(
        &mut self,
        visit_stack: &mut Vec<Coordinates>,
        coordinates: Coordinates,
    ) -> Cell {
        let new_cell = Cell::new(coordinates);
        self.maze.set_cell(new_cell);
        visit_stack.push(coordinates);
        new_cell
    }

    fn create_passage(
        &mut self,
        target_cell: &mut Cell,
        neighbor_cell: &mut Cell,
        direction: &Direction,
    ) {
        self.create_edge(target_cell, neighbor_cell, direction, CellEdge::Passage);
    }

    fn create_wall(
        &mut self,
        target_cell: &mut Cell,
        neighbor_cell: &mut Cell,
        direction: &Direction,
    ) {
        self.create_edge(target_cell, neighbor_cell, direction, CellEdge::Wall);
    }

    fn create_edge(
        &mut self,
        target_cell: &mut Cell,
        neighbor_cell: &mut Cell,
        direction: &Direction,
        cell_edge: CellEdge,
    ) {
        target_cell.set_edge(direction, Some(cell_edge));
        self.maze.set_cell(*target_cell);
        neighbor_cell.set_edge(&direction.opposite(), Some(cell_edge));
        self.maze.set_cell(*neighbor_cell);
    }

    fn create_border(&mut self, cell: &mut Cell, direction: &Direction) {
        cell.set_edge(direction, Some(CellEdge::Border));
        self.maze.set_cell(*cell);
    }
}
