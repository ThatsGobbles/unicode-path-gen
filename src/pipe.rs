use crate::grid::PipeCell;

pub struct PipeGrid {
    grid: Vec<PipeCell>,
    x_length: usize,
    y_length: usize,
}

impl PipeGrid {
    pub fn new(x_length: usize, y_length: usize) -> Self {
        let grid = vec![PipeCell::default(); x_length * y_length];
        Self { grid, x_length, y_length, }
    }

    #[inline]
    pub fn x_length(&self) -> usize {
        self.x_length
    }

    #[inline]
    pub fn y_length(&self) -> usize {
        self.y_length
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&PipeCell> {
        if x < self.x_length() && y < self.y_length() {
            self.grid.get(x * y)
        }
        else { None }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut PipeCell> {
        if x < self.x_length() && y < self.y_length() {
            self.grid.get_mut(x * y)
        }
        else { None }
    }

    pub fn clear(&mut self) {
        for c in self.grid.iter_mut() {
            c.clear();
        }
    }
}
