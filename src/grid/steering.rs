#[derive(Copy, Clone)]
pub enum Steering {
    Straight,
    Left,
    Right,
}

impl Steering {
    pub fn flip(&self) -> Self {
        match self {
            &Steering::Left => Steering::Right,
            &Steering::Right => Steering::Left,
            &Steering::Straight => Steering::Straight,
        }
    }
}
