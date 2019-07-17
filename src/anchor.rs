use crate::grid::Coordinate;
use crate::grid::Direction;
use crate::grid::Position;

#[derive(Copy, Clone)]
pub enum Anchor {
    North(Coordinate),
    South(Coordinate),
    West(Coordinate),
    East(Coordinate),
}

impl Anchor {
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        match self {
            &Anchor::North(Coordinate::Pos(x)) => x < width,
            &Anchor::South(Coordinate::Pos(x)) => x < width,
            &Anchor::West(Coordinate::Pos(y)) => y < height,
            &Anchor::East(Coordinate::Pos(y)) => y < height,
            _ => false,
        }
    }

    pub fn direction(&self) -> Direction {
        match self {
            &Anchor::North(..) => Direction::Down,
            &Anchor::South(..) => Direction::Up,
            &Anchor::West(..) => Direction::Right,
            &Anchor::East(..) => Direction::Left,
        }
    }

    pub fn position(&self, width: usize, height: usize) -> Position {
        match self {
            &Anchor::North(x) => (x.into(), 0.into()),
            &Anchor::South(x) => (x.into(), Coordinate::from(height).sub(1)),
            &Anchor::West(y) => (0.into(), y.into()),
            &Anchor::East(y) => (Coordinate::from(width).sub(1), y.into()),
        }.into()
    }
}
