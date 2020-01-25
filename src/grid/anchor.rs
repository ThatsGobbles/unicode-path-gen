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

#[cfg(test)]
mod tests {
    use super::*;

    use strum::IntoEnumIterator;

    #[test]
    fn in_bounds() {
        for direction in Direction::iter() {
            for x_length in 0usize..=9 {
                for y_length in 0usize..=9 {
                    for a in 0usize..=5 {
                        let co_pos = Coordinate::Pos(a);
                        let co_neg = Coordinate::Neg(a);

                        let anchor_edge_pos = Anchor::Edge(direction, co_pos);
                        let anchor_edge_neg = Anchor::Edge(direction, co_neg);

                        let exp_result = match direction {
                            Direction::North | Direction::South => a < x_length,
                            Direction::West | Direction::East => a < y_length,
                        };

                        assert_eq!(anchor_edge_pos.in_bounds(x_length, y_length), exp_result);
                        assert_eq!(anchor_edge_neg.in_bounds(x_length, y_length), false);

                        for b in 0usize..=5 {
                            let co_x_pos = Coordinate::Pos(a);
                            let co_x_neg = Coordinate::Neg(a);
                            let co_y_pos = Coordinate::Pos(b);
                            let co_y_neg = Coordinate::Neg(b);

                            let pos_pp = Position::new(co_x_pos, co_y_pos);
                            let pos_pn = Position::new(co_x_pos, co_y_neg);
                            let pos_np = Position::new(co_x_neg, co_y_pos);
                            let pos_nn = Position::new(co_x_neg, co_y_neg);

                            let anchor_cell_pp = Anchor::Cell(direction, pos_pp);
                            let anchor_cell_pn = Anchor::Cell(direction, pos_pn);
                            let anchor_cell_np = Anchor::Cell(direction, pos_np);
                            let anchor_cell_nn = Anchor::Cell(direction, pos_nn);

                            assert_eq!(anchor_cell_pp.in_bounds(x_length, y_length), a < x_length && b < y_length);
                            assert_eq!(anchor_cell_pn.in_bounds(x_length, y_length), false);
                            assert_eq!(anchor_cell_np.in_bounds(x_length, y_length), false);
                            assert_eq!(anchor_cell_nn.in_bounds(x_length, y_length), false);
                        }
                    }
                }
            }
        }
    }

    // #[test]
    // fn position() {
    //     for direction in Direction::iter() {
    //         for x_length in 0usize..=9 {
    //             for y_length in 0usize..=9 {
    //                 for a in 0usize..=5 {
    //                     let co_pos = Coordinate::Pos(a);
    //                     let co_neg = Coordinate::Neg(a);

    //                     let anchor_edge_pos = Anchor::Edge(direction, co_pos);
    //                     let anchor_edge_neg = Anchor::Edge(direction, co_neg);

    //                     let (co_xp, co_yp, co_xn, co_yn) = match direction {
    //                         Direction::North => (co_pos, 0.into(), co_neg, 0.into()),
    //                         Direction::South => (co_pos, y_length.into() - 1, co_neg, y_length.into() - 1),
    //                         Direction::West => (0.into(), co_pos, 0.into(), co_neg),
    //                         Direction::East => (x_length.into() - 1, co_pos, x_length.into() - 1, co_neg),
    //                     };

    //                     let exp_pos_pos = Position::new(co_xp, co_yp);

    //                     assert_eq!(anchor_edge_pos.position(x_length, y_length), exp_result);
    //                     assert_eq!(anchor_edge_neg.position(x_length, y_length), false);

    //                     for b in 0usize..=5 {
    //                         let co_x_pos = Coordinate::Pos(a);
    //                         let co_x_neg = Coordinate::Neg(a);
    //                         let co_y_pos = Coordinate::Pos(b);
    //                         let co_y_neg = Coordinate::Neg(b);

    //                         let pos_pp = Position::new(co_x_pos, co_y_pos);
    //                         let pos_pn = Position::new(co_x_pos, co_y_neg);
    //                         let pos_np = Position::new(co_x_neg, co_y_pos);
    //                         let pos_nn = Position::new(co_x_neg, co_y_neg);

    //                         let anchor_cell_pp = Anchor::Cell(direction, pos_pp);
    //                         let anchor_cell_pn = Anchor::Cell(direction, pos_pn);
    //                         let anchor_cell_np = Anchor::Cell(direction, pos_np);
    //                         let anchor_cell_nn = Anchor::Cell(direction, pos_nn);

    //                         assert_eq!(anchor_cell_pp.position(x_length, y_length), a < x_length && b < y_length);
    //                         assert_eq!(anchor_cell_pn.position(x_length, y_length), false);
    //                         assert_eq!(anchor_cell_np.position(x_length, y_length), false);
    //                         assert_eq!(anchor_cell_nn.position(x_length, y_length), false);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
