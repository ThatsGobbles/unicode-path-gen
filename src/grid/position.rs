use crate::grid::Coordinate;
use crate::grid::Direction;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: Coordinate,
    y: Coordinate,
}

impl From<(Coordinate, Coordinate)> for Position {
    fn from((x, y): (Coordinate, Coordinate)) -> Self {
        Self { x, y, }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        let x: Coordinate = x.into();
        let y: Coordinate = y.into();
        Self::from((x, y))
    }
}

impl Position {
    pub fn shift(&self, direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::Up       => (self.x, self.y - 1),
            Direction::Down     => (self.x, self.y + 1),
            Direction::Left     => (self.x - 1, self.y),
            Direction::Right    => (self.x + 1, self.y),
        };

        Self { x, y, }
    }

    pub fn flip_x(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }

    pub fn flip_y(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }

    pub fn flip(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.x.in_bounds(width) && self.y.in_bounds(height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pp(x: usize, y: usize) -> Position {
        let x = Coordinate::Pos(x);
        let y = Coordinate::Pos(y);
        Position { x, y, }
    }

    fn pn(x: usize, y: usize) -> Position {
        let x = Coordinate::Pos(x);
        let y = Coordinate::Neg(y);
        Position { x, y, }
    }

    fn np(x: usize, y: usize) -> Position {
        let x = Coordinate::Neg(x);
        let y = Coordinate::Pos(y);
        Position { x, y, }
    }

    fn nn(x: usize, y: usize) -> Position {
        let x = Coordinate::Neg(x);
        let y = Coordinate::Neg(y);
        Position { x, y, }
    }

    #[test]
    fn position_shift() {
        let pos_p3_p2 = Position { x: Coordinate::Pos(3), y: Coordinate::Pos(2) };
        let pos_n3_n2 = Position { x: Coordinate::Neg(3), y: Coordinate::Neg(2) };
        let pos_p0_p0 = Position { x: Coordinate::Pos(0), y: Coordinate::Pos(0) };
        let pos_n0_n0 = Position { x: Coordinate::Neg(0), y: Coordinate::Neg(0) };

        let inputs_and_expected = vec![
            ((pos_p3_p2, Direction::Up), pp(3, 1)),
            ((pos_p3_p2, Direction::Down), pp(3, 3)),
            ((pos_p3_p2, Direction::Left), pp(2, 2)),
            ((pos_p3_p2, Direction::Right), pp(4, 2)),
            ((pos_n3_n2, Direction::Up), nn(3, 3)),
            ((pos_n3_n2, Direction::Down), nn(3, 1)),
            ((pos_n3_n2, Direction::Left), nn(4, 2)),
            ((pos_n3_n2, Direction::Right), nn(2, 2)),
            ((pos_p0_p0, Direction::Up), pn(0, 0)),
            ((pos_p0_p0, Direction::Down), pp(0, 1)),
            ((pos_p0_p0, Direction::Left), np(0, 0)),
            ((pos_p0_p0, Direction::Right), pp(1, 0)),
            ((pos_n0_n0, Direction::Up), nn(0, 1)),
            ((pos_n0_n0, Direction::Down), np(0, 0)),
            ((pos_n0_n0, Direction::Left), nn(1, 0)),
            ((pos_n0_n0, Direction::Right), pn(0, 0)),
        ];

        for (inputs, expected) in inputs_and_expected {
            let (input_a, input_b) = inputs;
            let produced = input_a.shift(input_b);
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn position_flip_x() {
        let pos_p3_p2 = Position { x: Coordinate::Pos(3), y: Coordinate::Pos(2) };
        let pos_p3_n2 = Position { x: Coordinate::Pos(3), y: Coordinate::Neg(2) };
        let pos_n3_p2 = Position { x: Coordinate::Neg(3), y: Coordinate::Pos(2) };
        let pos_n3_n2 = Position { x: Coordinate::Neg(3), y: Coordinate::Neg(2) };
        let pos_p0_p0 = Position { x: Coordinate::Pos(0), y: Coordinate::Pos(0) };
        let pos_n0_n0 = Position { x: Coordinate::Neg(0), y: Coordinate::Neg(0) };

        let inputs_and_expected = vec![
            (pos_p3_p2, np(3, 2)),
            (pos_p3_n2, nn(3, 2)),
            (pos_n3_p2, pp(3, 2)),
            (pos_n3_n2, pn(3, 2)),
            (pos_p0_p0, np(0, 0)),
            (pos_n0_n0, pn(0, 0)),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = input.flip_x();
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn position_flip_y() {
        let pos_p3_p2 = Position { x: Coordinate::Pos(3), y: Coordinate::Pos(2) };
        let pos_p3_n2 = Position { x: Coordinate::Pos(3), y: Coordinate::Neg(2) };
        let pos_n3_p2 = Position { x: Coordinate::Neg(3), y: Coordinate::Pos(2) };
        let pos_n3_n2 = Position { x: Coordinate::Neg(3), y: Coordinate::Neg(2) };
        let pos_p0_p0 = Position { x: Coordinate::Pos(0), y: Coordinate::Pos(0) };
        let pos_n0_n0 = Position { x: Coordinate::Neg(0), y: Coordinate::Neg(0) };

        let inputs_and_expected = vec![
            (pos_p3_p2, pn(3, 2)),
            (pos_p3_n2, pp(3, 2)),
            (pos_n3_p2, nn(3, 2)),
            (pos_n3_n2, np(3, 2)),
            (pos_p0_p0, pn(0, 0)),
            (pos_n0_n0, np(0, 0)),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = input.flip_y();
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn position_flip() {
        let pos_p3_p2 = Position { x: Coordinate::Pos(3), y: Coordinate::Pos(2) };
        let pos_p3_n2 = Position { x: Coordinate::Pos(3), y: Coordinate::Neg(2) };
        let pos_n3_p2 = Position { x: Coordinate::Neg(3), y: Coordinate::Pos(2) };
        let pos_n3_n2 = Position { x: Coordinate::Neg(3), y: Coordinate::Neg(2) };
        let pos_p0_p0 = Position { x: Coordinate::Pos(0), y: Coordinate::Pos(0) };
        let pos_n0_n0 = Position { x: Coordinate::Neg(0), y: Coordinate::Neg(0) };

        let inputs_and_expected = vec![
            (pos_p3_p2, nn(3, 2)),
            (pos_p3_n2, np(3, 2)),
            (pos_n3_p2, pn(3, 2)),
            (pos_n3_n2, pp(3, 2)),
            (pos_p0_p0, nn(0, 0)),
            (pos_n0_n0, pp(0, 0)),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = input.flip();
            assert_eq!(expected, produced);
        }
    }
}
