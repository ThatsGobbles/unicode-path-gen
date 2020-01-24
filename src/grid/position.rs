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
    fn flip_x() {
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

                assert_eq!(pos_pp.flip_x(), Position::new(-xp, yp));
                assert_eq!(pos_pn.flip_x(), Position::new(-xp, yn));
                assert_eq!(pos_np.flip_x(), Position::new(-xn, yp));
                assert_eq!(pos_nn.flip_x(), Position::new(-xn, yn));
            }
        }
    }

    #[test]
    fn flip_y() {
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

                assert_eq!(pos_pp.flip_y(), Position::new(xp, -yp));
                assert_eq!(pos_pn.flip_y(), Position::new(xp, -yn));
                assert_eq!(pos_np.flip_y(), Position::new(xn, -yp));
                assert_eq!(pos_nn.flip_y(), Position::new(xn, -yn));
            }
        }
    }

    #[test]
    fn in_bounds() {
        for x_bound in 0usize..=9 {
            for y_bound in 0usize..=9 {
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

                        assert_eq!(pos_pp.in_bounds(x_bound, y_bound), x_raw < x_bound && y_raw < y_bound);
                        assert_eq!(pos_pn.in_bounds(x_bound, y_bound), false);
                        assert_eq!(pos_np.in_bounds(x_bound, y_bound), false);
                        assert_eq!(pos_nn.in_bounds(x_bound, y_bound), false);
                    }
                }
            }
        }
    }
}
