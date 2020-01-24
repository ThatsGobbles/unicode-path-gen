/// Represents each of the ways that a path can exit the center of a character cell.
#[derive(Copy, Clone)]
pub enum Steering {
    Straight,
    Left,
    Right,
}

impl Steering {
    /// Inverts this `Steering`, such that left and right are reversed.
    pub fn invert(&self) -> Self {
        match self {
            &Self::Left => Self::Right,
            &Self::Right => Self::Left,
            &Self::Straight => Self::Straight,
        }
    }
}
