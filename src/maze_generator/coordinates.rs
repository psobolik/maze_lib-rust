/// Represents a location in a maze as column and row.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinates {
    column: i32,
    row: i32,
}

/// Associated functions to create and use a Coordinates struct.
impl Coordinates {
    /// Coordinates struct that can be added to another coordinate to get the
    /// coordinates above it.
    pub const UP: Coordinates = Coordinates { column: 0, row: 1 };
    /// Coordinates struct that can be added to another coordinate to get the
    /// coordinates to the right of it.
    pub const RIGHT: Coordinates = Coordinates { column: 1, row: 0 };
    /// Coordinates struct that can be added to another coordinate to get the
    /// coordinates below it.
    pub const DOWN: Coordinates = Coordinates { column: 0, row: -1 };
    /// Coordinates struct that can be added to another coordinate to get the
    /// coordinates to the left of it.
    pub const LEFT: Coordinates = Coordinates { column: -1, row: 0 };

    /// Creates a new Coordinates struct with the given column and row.
    pub fn new(column: i32, row: i32) -> Coordinates {
        Coordinates { column, row }
    }

    /// Returns the Coordinates struct's row.
    pub fn row(&self) -> i32 {
        self.row
    }

    /// Returns the Coordinates struct's column.
    pub fn column(&self) -> i32 {
        self.column
    }
}

impl std::ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, increment: Coordinates) -> Coordinates {
        Coordinates {
            column: self.column + increment.column,
            row: self.row + increment.row,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_up_moves_up() {
        let before = Coordinates::new(2, 2);
        let after = before + Coordinates::UP;
        let expected = Coordinates::new(2, 3);
        assert_eq!(after, expected)
    }

    #[test]
    fn adding_right_moves_right() {
        let before = Coordinates::new(2, 2);
        let after = before + Coordinates::RIGHT;
        let expected = Coordinates::new(3, 2);
        assert_eq!(after, expected)
    }

    #[test]
    fn adding_down_moves_down() {
        let before = Coordinates::new(2, 2);
        let after = before + Coordinates::DOWN;
        let expected = Coordinates::new(2, 1);
        assert_eq!(after, expected)
    }

    #[test]
    fn adding_left_moves_left() {
        let before = Coordinates::new(2, 2);
        let after = before + Coordinates::LEFT;
        let expected = Coordinates::new(1, 2);
        assert_eq!(after, expected)
    }

    #[test]
    fn handles_format() {
        let coordinates = Coordinates::new(4, 2);
        assert_eq!(
            format!("{coordinates:?}"),
            "Coordinates { column: 4, row: 2 }"
        )
    }
}
