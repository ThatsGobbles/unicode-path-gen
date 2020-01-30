use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::grid::Coordinate;
use crate::grid::Direction;

/// A two-dimensional, zero-indexed position into a grid of character cells.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: Coordinate,
    y: Coordinate,
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
            Direction::North => (self.x, self.y - n),
            Direction::East => (self.x + n, self.y),
            Direction::South => (self.x, self.y + n),
            Direction::West => (self.x - n, self.y),
        };

        Self { x, y, }
    }

    /// Checks if this `Position` is inside a two-dimensional bounding region,
    /// from [(0, 0), (x_length, y_length)).
    pub fn in_bounds(&self, x_length: usize, y_length: usize) -> bool {
        self.x.in_bounds(x_length) && self.y.in_bounds(y_length)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use strum::IntoEnumIterator;

    #[test]
    fn shift() {
        for shift_amount in 0usize..=5 {
            for direction in Direction::iter() {
                for x_raw in 0usize..=5 {
                    for y_raw in 0usize..=5 {
                        let xp = Coordinate::Pos(x_raw);
                        let yp = Coordinate::Pos(y_raw);
                        let xn = Coordinate::Neg(x_raw);
                        let yn = Coordinate::Neg(y_raw);

                        let pos_pp = Position::new(xp, yp);
                        let pos_pn = Position::new(xp, yn);
                        let pos_np = Position::new(xn, yp);
                        let pos_nn = Position::new(xn, yn);

                        let (exp_xp, exp_xn, exp_yp, exp_yn) = match direction {
                            Direction::North => (xp, xn, yp - shift_amount, yn - shift_amount),
                            Direction::East => (xp + shift_amount, xn + shift_amount, yp, yn),
                            Direction::South => (xp, xn, yp + shift_amount, yn + shift_amount),
                            Direction::West => (xp - shift_amount, xn - shift_amount, yp, yn),
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
    fn in_bounds() {
        for x_length in 0usize..=9 {
            for y_length in 0usize..=9 {
                for x_raw in 0usize..=5 {
                    for y_raw in 0usize..=5 {
                        let xp = Coordinate::Pos(x_raw);
                        let yp = Coordinate::Pos(y_raw);
                        let xn = Coordinate::Neg(x_raw);
                        let yn = Coordinate::Neg(y_raw);

                        let pos_pp = Position::new(xp, yp);
                        let pos_pn = Position::new(xp, yn);
                        let pos_np = Position::new(xn, yp);
                        let pos_nn = Position::new(xn, yn);

                        assert_eq!(pos_pp.in_bounds(x_length, y_length), x_raw < x_length && y_raw < y_length);
                        assert_eq!(pos_pn.in_bounds(x_length, y_length), false);
                        assert_eq!(pos_np.in_bounds(x_length, y_length), false);
                        assert_eq!(pos_nn.in_bounds(x_length, y_length), false);
                    }
                }
            }
        }
    }
}
