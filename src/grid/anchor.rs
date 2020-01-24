use crate::grid::Coordinate;
use crate::grid::Direction;
use crate::grid::Position;

#[derive(Copy, Clone)]
pub enum Anchor {
    // At the edge of a two-dimensional bounding box.
    Edge(Direction, Coordinate),
    // At the center of a cell.
    Cell(Direction, Position),
}

impl Anchor {
    /// Checks if this `Anchor` is in bounds with respect to a two-dimensional
    /// target region.
    pub fn in_bounds(&self, x_length: usize, y_length: usize) -> bool {
        match self {
            Self::Edge(dir, co) => {
                match dir {
                    Direction::North | Direction::South => co.in_bounds(x_length),
                    Direction::West | Direction::East => co.in_bounds(y_length),
                }
            },
            Self::Cell(_, pos) => pos.in_bounds(x_length, y_length),
        }
    }

    /// Returns the "heading" `Direction` of this `Anchor`.
    /// This is the `Direction` that a path would be facing were it to start
    /// from this `Anchor`.
    pub fn heading(&self) -> Direction {
        match self {
            Self::Edge(dir, _) => dir.invert(),
            Self::Cell(dir, _) => *dir,
        }
    }

    /// Returns the `Position` of this `Anchor` with respect to a two-dimensional
    /// target region. Note that the returned `Position` may be out of bounds.
    pub fn position(&self, x_length: usize, y_length: usize) -> Position {
        match self {
            Self::Edge(dir, co) => {
                let (x_co, y_co) = match dir {
                    Direction::North => (*co, Coordinate::Pos(0)),
                    Direction::South => (*co, Coordinate::Pos(y_length) - 1),
                    Direction::West => (Coordinate::Pos(0), *co),
                    Direction::East => (Coordinate::Pos(x_length) - 1, *co),
                };

                Position::new(x_co, y_co)
            },
            Self::Cell(_, pos) => *pos,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn in_bounds() {
//         for x_bound in 0usize..=9 {
//             for y_bound in 0usize..=9 {
//                 for a in 0usize..=5 {
//                     let ap = Coordinate::Pos(a);
//                     let an = Coordinate::Neg(a);

//                     assert_eq!(Anchor::North(ap).in_bounds(x_bound, y_bound), a < x_bound);
//                     assert_eq!(Anchor::South(ap).in_bounds(x_bound, y_bound), a < x_bound);
//                     assert_eq!(Anchor::West(ap).in_bounds(x_bound, y_bound), a < y_bound);
//                     assert_eq!(Anchor::East(ap).in_bounds(x_bound, y_bound), a < y_bound);

//                     assert_eq!(Anchor::North(an).in_bounds(x_bound, y_bound), false);
//                     assert_eq!(Anchor::South(an).in_bounds(x_bound, y_bound), false);
//                     assert_eq!(Anchor::West(an).in_bounds(x_bound, y_bound), false);
//                     assert_eq!(Anchor::East(an).in_bounds(x_bound, y_bound), false);
//                 }
//             }
//         }
//     }

//     #[test]
//     fn position() {
//         for x_bound in 0usize..=9 {
//             for y_bound in 0usize..=9 {
//                 for a in 0usize..=5 {
//                     let ap = Coordinate::Pos(a);
//                     let an = Coordinate::Neg(a);

//                     assert_eq!(
//                         Anchor::North(ap).position(x_bound, y_bound),
//                         Position::new(ap, 0.into()),
//                     );
//                     assert_eq!(
//                         Anchor::South(ap).position(x_bound, y_bound),
//                         Position::new(ap, Coordinate::from(y_bound) - 1),
//                     );
//                     assert_eq!(
//                         Anchor::West(ap).position(x_bound, y_bound),
//                         Position::new(0.into(), ap),
//                     );
//                     assert_eq!(
//                         Anchor::East(ap).position(x_bound, y_bound),
//                         Position::new(Coordinate::from(x_bound) - 1, ap),
//                     );

//                     assert_eq!(
//                         Anchor::North(an).position(x_bound, y_bound),
//                         Position::new(an, 0.into()),
//                     );
//                     assert_eq!(
//                         Anchor::South(an).position(x_bound, y_bound),
//                         Position::new(an, Coordinate::from(y_bound) - 1),
//                     );
//                     assert_eq!(
//                         Anchor::West(an).position(x_bound, y_bound),
//                         Position::new(0.into(), an),
//                     );
//                     assert_eq!(
//                         Anchor::East(an).position(x_bound, y_bound),
//                         Position::new(Coordinate::from(x_bound) - 1, an),
//                     );
//                 }
//             }
//         }
//     }
// }
