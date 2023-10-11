use super::{cell_edge::CellEdge, direction::Direction};
use crate::maze::coordinates::Coordinates;
use rand::Rng;

/// Represents a cell in a maze, with coordinates and edges.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    coordinates: Coordinates,
    north_edge: Option<CellEdge>,
    east_edge: Option<CellEdge>,
    south_edge: Option<CellEdge>,
    west_edge: Option<CellEdge>,
}

/// Associated functions to create and use a Cell struct.
impl Cell {
    /// Creates a new Cell with the given coordinates. The new cell has no assigned edges.
    pub fn new(coordinates: Coordinates) -> Cell {
        Cell {
            coordinates,
            north_edge: None,
            east_edge: None,
            south_edge: None,
            west_edge: None,
        }
    }

    /// Returns true if all of the Cell's edges have been assigned.
    pub fn is_fully_assigned(&self) -> bool {
        self.unassigned_directions() == vec![]
    }

    /// Returns a vector containing the Cell's unassigned edges.
    pub fn unassigned_directions(&self) -> Vec<Direction> {
        let mut vec = Vec::new();
        if self.north_edge.is_none() {
            vec.push(Direction::North);
        }
        if self.east_edge.is_none() {
            vec.push(Direction::East);
        }
        if self.south_edge.is_none() {
            vec.push(Direction::South);
        }
        if self.west_edge.is_none() {
            vec.push(Direction::West);
        }
        vec
    }

    /// Returns the Cell's coordinates.
    pub fn coordinates(self) -> Coordinates {
        self.coordinates
    }

    /// Returns the value of the Cell's edge in the given direction.
    pub fn edge(&self, direction: &Direction) -> Option<CellEdge> {
        match direction {
            Direction::North => self.north_edge,
            Direction::East => self.east_edge,
            Direction::South => self.south_edge,
            Direction::West => self.west_edge,
        }
    }

    /// Sets the Cell's edge in the given direction to the given value.
    pub fn set_edge(&mut self, direction: &Direction, cell_edge: Option<CellEdge>) {
        match direction {
            Direction::North => self.north_edge = cell_edge,
            Direction::East => self.east_edge = cell_edge,
            Direction::South => self.south_edge = cell_edge,
            Direction::West => self.west_edge = cell_edge,
        }
    }

    /// Returns the direction of one of the Cell's unassigned edges.
    pub fn random_unassigned_direction(&self) -> Direction {
        let unassigned_directions = self.unassigned_directions();
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..unassigned_directions.len());
        unassigned_directions[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unassigned_directions_none_assigned() {
        let cell = Cell::new(Coordinates::new(0, 0));
        assert_eq!(
            cell.unassigned_directions(),
            vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
        )
    }

    #[test]
    fn unassigned_directions_one_assigned() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        cell.set_edge(&Direction::East, Some(CellEdge::Border));
        cell.set_edge(&Direction::North, None);
        assert_eq!(
            cell.unassigned_directions(),
            vec![Direction::North, Direction::South, Direction::West]
        )
    }

    #[test]
    fn unassigned_directions_two_assigned() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        cell.set_edge(&Direction::North, Some(CellEdge::Border));
        cell.set_edge(&Direction::West, Some(CellEdge::Passage));
        assert_eq!(
            cell.unassigned_directions(),
            vec![Direction::East, Direction::South]
        )
    }

    #[test]
    fn unassigned_directions_three_assigned() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        cell.set_edge(&Direction::North, Some(CellEdge::Border));
        cell.set_edge(&Direction::South, Some(CellEdge::Wall));
        cell.set_edge(&Direction::West, Some(CellEdge::Passage));
        assert_eq!(cell.unassigned_directions(), vec![Direction::East])
    }

    #[test]
    fn unassigned_directions_all_assigned() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        cell.set_edge(&Direction::North, Some(CellEdge::Border));
        cell.set_edge(&Direction::East, Some(CellEdge::Wall));
        cell.set_edge(&Direction::South, Some(CellEdge::Passage));
        cell.set_edge(&Direction::West, Some(CellEdge::Passage));
        assert_eq!(cell.unassigned_directions(), vec![])
    }

    #[test]
    fn is_fully_assigned_directions_none_assigned() {
        let cell = Cell::new(Coordinates::new(0, 0));
        assert!(!cell.is_fully_assigned())
    }

    #[test]
    fn is_fully_assigned_directions_one_assigned() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        cell.set_edge(&Direction::North, Some(CellEdge::Border));
        assert!(!cell.is_fully_assigned())
    }

    #[test]
    fn is_fully_assigned_directions_two_assigned() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        cell.set_edge(&Direction::East, Some(CellEdge::Border));
        cell.set_edge(&Direction::West, Some(CellEdge::Passage));
        assert!(!cell.is_fully_assigned())
    }

    #[test]
    fn is_fully_assigned_directions_three_assigned() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        cell.set_edge(&Direction::East, Some(CellEdge::Border));
        cell.set_edge(&Direction::South, Some(CellEdge::Wall));
        cell.set_edge(&Direction::West, Some(CellEdge::Passage));
        assert!(!cell.is_fully_assigned())
    }

    #[test]
    fn is_fully_assigned_directions_all_assigned() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        cell.set_edge(&Direction::North, Some(CellEdge::Border));
        cell.set_edge(&Direction::East, Some(CellEdge::Wall));
        cell.set_edge(&Direction::South, Some(CellEdge::Passage));
        cell.set_edge(&Direction::West, Some(CellEdge::Passage));
        assert!(cell.is_fully_assigned())
    }

    #[test]
    fn get_and_set_north() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        let edge = Some(CellEdge::Passage);
        let direction = Direction::North;
        cell.set_edge(&direction, edge);
        assert_eq!(cell.edge(&direction), edge);
    }

    #[test]
    fn get_and_set_east() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        let edge = Some(CellEdge::Passage);
        let direction = Direction::East;
        cell.set_edge(&direction, edge);
        assert_eq!(cell.edge(&direction), edge);
    }

    #[test]
    fn get_and_set_south() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        let edge = Some(CellEdge::Passage);
        let direction = Direction::South;
        cell.set_edge(&direction, edge);
        assert_eq!(cell.edge(&direction), edge);
    }

    #[test]
    fn get_and_set_west() {
        let mut cell = Cell::new(Coordinates::new(0, 0));
        let edge = Some(CellEdge::Passage);
        let direction = Direction::West;
        cell.set_edge(&direction, edge);
        assert_eq!(cell.edge(&direction), edge);
    }
}
