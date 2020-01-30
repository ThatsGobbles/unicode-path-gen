use strum_macros::EnumIter;

use crate::grid::Steering;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Inverts this `Direction`.
    pub fn invert(&self) -> Self {
        match self {
            &Self::North => Self::South,
            &Self::East => Self::West,
            &Self::South => Self::North,
            &Self::West => Self::East,
        }
    }

    /// Rotates this `Direction` to the left (anticlockwise).
    pub fn turn_left(&self) -> Self {
        match self {
            &Self::North => Self::West,
            &Self::East => Self::North,
            &Self::South => Self::East,
            &Self::West => Self::South,
        }
    }

    /// Rotates this `Direction` to the right (clockwise).
    pub fn turn_right(&self) -> Self {
        match self {
            &Self::North => Self::East,
            &Self::East => Self::South,
            &Self::South => Self::West,
            &Self::West => Self::North,
        }
    }

    /// Steers this `Direction` into a new `Direction`.
    pub fn steer(&self, s: Steering) -> Self {
        match s {
            Steering::Straight => *self,
            Steering::Left => self.turn_left(),
            Steering::Right => self.turn_right(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert() {
        assert_eq!(Direction::North.invert(), Direction::South);
        assert_eq!(Direction::East.invert(), Direction::West);
        assert_eq!(Direction::South.invert(), Direction::North);
        assert_eq!(Direction::West.invert(), Direction::East);
    }

    #[test]
    fn turn_left() {
        assert_eq!(Direction::North.turn_left(), Direction::West);
        assert_eq!(Direction::East.turn_left(), Direction::North);
        assert_eq!(Direction::South.turn_left(), Direction::East);
        assert_eq!(Direction::West.turn_left(), Direction::South);
    }

    #[test]
    fn turn_right() {
        assert_eq!(Direction::North.turn_right(), Direction::East);
        assert_eq!(Direction::East.turn_right(), Direction::South);
        assert_eq!(Direction::South.turn_right(), Direction::West);
        assert_eq!(Direction::West.turn_right(), Direction::North);
    }

    #[test]
    fn steer() {
        assert_eq!(Direction::North.steer(Steering::Straight), Direction::North);
        assert_eq!(Direction::East.steer(Steering::Straight), Direction::East);
        assert_eq!(Direction::South.steer(Steering::Straight), Direction::South);
        assert_eq!(Direction::West.steer(Steering::Straight), Direction::West);

        assert_eq!(Direction::North.steer(Steering::Left), Direction::West);
        assert_eq!(Direction::East.steer(Steering::Left), Direction::North);
        assert_eq!(Direction::South.steer(Steering::Left), Direction::East);
        assert_eq!(Direction::West.steer(Steering::Left), Direction::South);

        assert_eq!(Direction::North.steer(Steering::Right), Direction::East);
        assert_eq!(Direction::East.steer(Steering::Right), Direction::South);
        assert_eq!(Direction::South.steer(Steering::Right), Direction::West);
        assert_eq!(Direction::West.steer(Steering::Right), Direction::North);
    }
}
