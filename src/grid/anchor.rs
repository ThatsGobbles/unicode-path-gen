use crate::grid::Coordinate;
use crate::grid::Direction;
use crate::grid::Position;

/// Represents a point on the edge of a bounding box.
/// Usually used to indicate the entry point of a pipe chain from the edge of
/// the canvas.
#[derive(Copy, Clone)]
pub enum Anchor {
    North(Coordinate),
    South(Coordinate),
    West(Coordinate),
    East(Coordinate),
}

impl Anchor {
    /// Checks if this `Anchor` is in bounds with respect to a two-dimensional
    /// target region.
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        match self {
            &Self::North(Coordinate::Pos(x)) => x < width,
            &Self::South(Coordinate::Pos(x)) => x < width,
            &Self::West(Coordinate::Pos(y)) => y < height,
            &Self::East(Coordinate::Pos(y)) => y < height,
            _ => false,
        }
    }

    /// Returns the "normal" `Direction` of this `Anchor`.
    /// This is the `Direction` that a path would be facing were it to enter the
    /// canvas from this `Anchor`.
    pub fn normal(&self) -> Direction {
        match self {
            &Self::North(..) => Direction::Down,
            &Self::South(..) => Direction::Up,
            &Self::West(..) => Direction::Right,
            &Self::East(..) => Direction::Left,
        }
    }

    /// Returns the `Position` of this `Anchor` with respect to a two-dimensional
    /// target region. Note that the returned `Position` may be out of bounds.
    pub fn position(&self, width: usize, height: usize) -> Position {
        let (x_co, y_co) = match self {
            &Self::North(x) => (x, 0.into()),
            &Self::South(x) => (x, Coordinate::from(height) - 1),
            &Self::West(y) => (0.into(), y),
            &Self::East(y) => (Coordinate::from(width) - 1, y),
        };

        Position::new(x_co, y_co)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_bounds() {
        for x_bound in 0usize..=9 {
            for y_bound in 0usize..=9 {
                for a in 0usize..=5 {
                    let ap = Coordinate::Pos(a);
                    let an = Coordinate::Neg(a);

                    assert_eq!(Anchor::North(ap).in_bounds(x_bound, y_bound), a < x_bound);
                    assert_eq!(Anchor::South(ap).in_bounds(x_bound, y_bound), a < x_bound);
                    assert_eq!(Anchor::West(ap).in_bounds(x_bound, y_bound), a < y_bound);
                    assert_eq!(Anchor::East(ap).in_bounds(x_bound, y_bound), a < y_bound);

                    assert_eq!(Anchor::North(an).in_bounds(x_bound, y_bound), false);
                    assert_eq!(Anchor::South(an).in_bounds(x_bound, y_bound), false);
                    assert_eq!(Anchor::West(an).in_bounds(x_bound, y_bound), false);
                    assert_eq!(Anchor::East(an).in_bounds(x_bound, y_bound), false);
                }
            }
        }
    }

    #[test]
    fn position() {
        for x_bound in 0usize..=9 {
            for y_bound in 0usize..=9 {
                for a in 0usize..=5 {
                    let ap = Coordinate::Pos(a);
                    let an = Coordinate::Neg(a);

                    assert_eq!(
                        Anchor::North(ap).position(x_bound, y_bound),
                        Position::new(ap, 0.into()),
                    );
                    assert_eq!(
                        Anchor::South(ap).position(x_bound, y_bound),
                        Position::new(ap, Coordinate::from(y_bound) - 1),
                    );
                    assert_eq!(
                        Anchor::West(ap).position(x_bound, y_bound),
                        Position::new(0.into(), ap),
                    );
                    assert_eq!(
                        Anchor::East(ap).position(x_bound, y_bound),
                        Position::new(Coordinate::from(x_bound) - 1, ap),
                    );

                    assert_eq!(
                        Anchor::North(an).position(x_bound, y_bound),
                        Position::new(an, 0.into()),
                    );
                    assert_eq!(
                        Anchor::South(an).position(x_bound, y_bound),
                        Position::new(an, Coordinate::from(y_bound) - 1),
                    );
                    assert_eq!(
                        Anchor::West(an).position(x_bound, y_bound),
                        Position::new(0.into(), an),
                    );
                    assert_eq!(
                        Anchor::East(an).position(x_bound, y_bound),
                        Position::new(Coordinate::from(x_bound) - 1, an),
                    );
                }
            }
        }
    }
}
