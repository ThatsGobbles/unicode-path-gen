use crate::grid::Steering;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn flip(&self) -> Self {
        match self {
            &Direction::Up => Direction::Down,
            &Direction::Down => Direction::Up,
            &Direction::Left => Direction::Right,
            &Direction::Right => Direction::Left,
        }
    }

    pub fn steer(&self, s: Steering) -> Self {
        match s {
            Steering::Straight => *self,
            Steering::Left => {
                match self {
                    &Direction::Up => Direction::Left,
                    &Direction::Down => Direction::Right,
                    &Direction::Left => Direction::Down,
                    &Direction::Right => Direction::Up,
                }
            },
            Steering::Right => {
                match self {
                    &Direction::Up => Direction::Right,
                    &Direction::Down => Direction::Left,
                    &Direction::Left => Direction::Up,
                    &Direction::Right => Direction::Down,
                }
            },
        }
    }
}
