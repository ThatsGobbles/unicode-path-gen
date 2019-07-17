pub trait InBounds1D {
    fn in_bounds(&self, length: usize) -> bool;
}

pub trait InBounds2D {
    fn in_bounds(&self, width: usize, height: usize) -> bool;
}
