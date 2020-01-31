use crate::grid::Position;
use crate::grid::Direction;

pub enum Anchor {
    /// An anchoring at the border of a cell, heading inwards towards the cell's
    /// center.
    Border(Position, Direction),
    /// An anchoring at the center of a cell, heading outwards towards one of
    /// the cell's borders.
    Center(Position, Direction),
}

impl Anchor {
    pub fn position(&self) -> Position {
        match self {
            &Self::Border(pos, _) => pos,
            &Self::Center(pos, _) => pos,
        }
    }

    /// The `Direction` a path would initially travel in if it were to start at
    /// this `Anchor`.
    pub fn heading(&self) -> Direction {
        match self {
            &Self::Border(_, dir) => dir.invert(),
            &Self::Center(_, dir) => dir,
        }
    }

    pub fn is_central(&self) -> bool {
        match self {
            &Self::Border(..) => false,
            &Self::Center(..) => true,
        }
    }
}
