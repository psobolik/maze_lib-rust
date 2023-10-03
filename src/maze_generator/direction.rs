use crate::maze_generator::coordinates::Coordinates;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    pub fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

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
    fn right_of_north() {
        assert_eq!(Direction::North.right(), Direction::East)
    }

    #[test]
    fn right_of_east() {
        assert_eq!(Direction::East.right(), Direction::South)
    }

    #[test]
    fn right_of_south() {
        assert_eq!(Direction::South.right(), Direction::West)
    }

    #[test]
    fn right_of_west() {
        assert_eq!(Direction::West.right(), Direction::North)
    }

    #[test]
    fn left_of_north() {
        assert_eq!(Direction::North.left(), Direction::West)
    }

    #[test]
    fn left_of_east() {
        assert_eq!(Direction::East.left(), Direction::North)
    }

    #[test]
    fn left_of_south() {
        assert_eq!(Direction::South.left(), Direction::East)
    }

    #[test]
    fn left_of_west() {
        assert_eq!(Direction::West.left(), Direction::South)
    }

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
