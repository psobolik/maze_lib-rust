use crate::maze_generator::coordinates::Coordinates;

/// Represents directions in a maze.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// Associated functions to create and use a Direction enum.
impl Direction {
    /// Returns the Direction's opposite.
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    /// Returns the Direction as coordinates that could be added to another
    /// Coordinates struct to get the coordinates in the direction. (_What?_)
    pub fn coordinates(&self) -> Coordinates {
        match self {
            Direction::North => Coordinates::UP,
            Direction::East => Coordinates::RIGHT,
            Direction::South => Coordinates::DOWN,
            Direction::West => Coordinates::LEFT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_of_north() {
        assert_eq!(Direction::North.opposite(), Direction::South)
    }

    #[test]
    fn opposite_of_east() {
        assert_eq!(Direction::East.opposite(), Direction::West)
    }

    #[test]
    fn opposite_of_south() {
        assert_eq!(Direction::South.opposite(), Direction::North)
    }

    #[test]
    fn opposite_of_west() {
        assert_eq!(Direction::West.opposite(), Direction::East)
    }
}
