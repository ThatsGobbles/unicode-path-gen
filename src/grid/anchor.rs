use crate::grid::Position;
use crate::grid::Direction;

/// An anchoring at the border or center of a cell, heading in a specified direction.
#[derive(Clone, Copy, Debug)]
pub struct Anchor {
    position: Position,
    heading: Direction,
    is_centered: bool,
}

impl Anchor {
    pub fn new(position: Position, heading: Direction, is_centered: bool) -> Self {
        Self {
            position,
            heading,
            is_centered,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    /// The `Direction` a path would initially travel in if it were to start at
    /// this `Anchor`.
    pub fn heading(&self) -> Direction {
        self.heading
    }

    pub fn is_centered(&self) -> bool {
        self.is_centered
    }

    pub fn border(&self) -> Option<Direction> {
        if self.is_centered() { None }
        else { Some(self.heading().invert()) }
    }

    pub fn len_segments(&self) -> usize {
        self.is_centered() as usize
    }
}
