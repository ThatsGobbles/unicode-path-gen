use strum_macros::EnumIter;

use crate::grid::Steering;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Inverts this `Direction`.
    pub fn invert(&self) -> Self {
        match self {
            &Self::Up => Self::Down,
            &Self::Down => Self::Up,
            &Self::Left => Self::Right,
            &Self::Right => Self::Left,
        }
    }

    /// Rotates this `Direction` to the left (anticlockwise).
    pub fn turn_left(&self) -> Self {
        match self {
            &Self::Up => Self::Left,
            &Self::Down => Self::Right,
            &Self::Left => Self::Down,
            &Self::Right => Self::Up,
        }
    }

    /// Rotates this `Direction` to the right (clockwise).
    pub fn turn_right(&self) -> Self {
        match self {
            &Self::Up => Self::Right,
            &Self::Down => Self::Left,
            &Self::Left => Self::Up,
            &Self::Right => Self::Down,
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
        assert_eq!(Direction::Up.invert(), Direction::Down);
        assert_eq!(Direction::Down.invert(), Direction::Up);
        assert_eq!(Direction::Left.invert(), Direction::Right);
        assert_eq!(Direction::Right.invert(), Direction::Left);
    }

    #[test]
    fn turn_left() {
        assert_eq!(Direction::Up.turn_left(), Direction::Left);
        assert_eq!(Direction::Down.turn_left(), Direction::Right);
        assert_eq!(Direction::Left.turn_left(), Direction::Down);
        assert_eq!(Direction::Right.turn_left(), Direction::Up);
    }

    #[test]
    fn turn_right() {
        assert_eq!(Direction::Up.turn_right(), Direction::Right);
        assert_eq!(Direction::Down.turn_right(), Direction::Left);
        assert_eq!(Direction::Left.turn_right(), Direction::Up);
        assert_eq!(Direction::Right.turn_right(), Direction::Down);
    }

    #[test]
    fn steer() {
        assert_eq!(Direction::Up.steer(Steering::Straight), Direction::Up);
        assert_eq!(Direction::Down.steer(Steering::Straight), Direction::Down);
        assert_eq!(Direction::Left.steer(Steering::Straight), Direction::Left);
        assert_eq!(Direction::Right.steer(Steering::Straight), Direction::Right);

        assert_eq!(Direction::Up.steer(Steering::Left), Direction::Left);
        assert_eq!(Direction::Down.steer(Steering::Left), Direction::Right);
        assert_eq!(Direction::Left.steer(Steering::Left), Direction::Down);
        assert_eq!(Direction::Right.steer(Steering::Left), Direction::Up);

        assert_eq!(Direction::Up.steer(Steering::Right), Direction::Right);
        assert_eq!(Direction::Down.steer(Steering::Right), Direction::Left);
        assert_eq!(Direction::Left.steer(Steering::Right), Direction::Up);
        assert_eq!(Direction::Right.steer(Steering::Right), Direction::Down);
    }
}
