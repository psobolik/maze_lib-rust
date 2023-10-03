#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinates {
    column: i32,
    row: i32,
}

impl Coordinates {
    pub const UP: Coordinates = Coordinates { column: 0, row: 1};
    pub const RIGHT: Coordinates = Coordinates { column: 1, row: 0 };
    pub const DOWN: Coordinates = Coordinates { column: 0, row: -1 };
    pub const LEFT: Coordinates = Coordinates { column: -1, row: 0 };

    pub fn new(column: i32, row: i32) -> Coordinates {
        Coordinates { column, row }
    }
    pub fn row(&self) -> i32 {
        self.row
    }
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
        let expected = Coordinates::new(2, 3 );
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
