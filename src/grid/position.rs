use crate::grid::Coordinate;
use crate::grid::Direction;

/// A two-dimensional, zero-indexed position into a grid of character cells.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: Coordinate,
    y: Coordinate,
}

impl From<(Coordinate, Coordinate)> for Position {
    fn from((x, y): (Coordinate, Coordinate)) -> Self {
        Self::new(x, y)
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self::from_raw(x, y)
    }
}

impl Position {
    pub fn new(x: Coordinate, y: Coordinate) -> Self {
        Self { x, y, }
    }

    pub fn from_raw(xr: usize, yr: usize) -> Self {
        Self::new(xr.into(), yr.into())
    }

    /// Shift this `Position` a number of units in the specified `Direction`.
    pub fn shift(&self, direction: Direction, n: usize) -> Self {
        let (x, y) = match direction {
            Direction::Up       => (self.x, self.y - n),
            Direction::Down     => (self.x, self.y + n),
            Direction::Left     => (self.x - n, self.y),
            Direction::Right    => (self.x + n, self.y),
        };

        Self { x, y, }
    }

    /// Flips this `Position` along the x-axis.
    pub fn flip_x(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }

    /// Flips this `Position` along the y-axis.
    pub fn flip_y(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }

    /// Checks if this `Position` is inside a two-dimensional bounding region,
    /// from [(0, 0), (width, length)).
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.x.in_bounds(width) && self.y.in_bounds(height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use strum::IntoEnumIterator;

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
    fn shift() {
        for shift_amount in 0usize..=3 {
            for direction in Direction::iter() {
                for x_raw in 0usize..=1 {
                    for y_raw in 0usize..=1 {
                        let xp = Coordinate::Pos(x_raw);
                        let yp = Coordinate::Pos(y_raw);
                        let xn = Coordinate::Neg(x_raw);
                        let yn = Coordinate::Neg(y_raw);

                        let pos_pp = Position::new(xp, yp);
                        let pos_pn = Position::new(xp, yn);
                        let pos_np = Position::new(xn, yp);
                        let pos_nn = Position::new(xn, yn);

                        let (exp_xp, exp_xn, exp_yp, exp_yn) = match direction {
                            Direction::Left => (xp - shift_amount, xn - shift_amount, yp, yn),
                            Direction::Right => (xp + shift_amount, xn + shift_amount, yp, yn),
                            Direction::Up => (xp, xn, yp - shift_amount, yn - shift_amount),
                            Direction::Down => (xp, xn, yp + shift_amount, yn + shift_amount),
                        };

                        assert_eq!(pos_pp.shift(direction, shift_amount), Position::new(exp_xp, exp_yp));
                        assert_eq!(pos_pn.shift(direction, shift_amount), Position::new(exp_xp, exp_yn));
                        assert_eq!(pos_np.shift(direction, shift_amount), Position::new(exp_xn, exp_yp));
                        assert_eq!(pos_nn.shift(direction, shift_amount), Position::new(exp_xn, exp_yn));
                    }
                }
            }
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
}
