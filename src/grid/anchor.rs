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
                    Direction::Up | Direction::Down => co.in_bounds(x_length),
                    Direction::Left | Direction::Right => co.in_bounds(y_length),
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
                    Direction::Up => (*co, Coordinate::Pos(0)),
                    Direction::Down => (*co, Coordinate::Pos(y_length) - 1),
                    Direction::Left => (Coordinate::Pos(0), *co),
                    Direction::Right => (Coordinate::Pos(x_length) - 1, *co),
                };

                Position::new(x_co, y_co)
            },
            Self::Cell(_, pos) => *pos,
        }
    }
}
