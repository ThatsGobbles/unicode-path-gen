use crate::grid::Steering;
use crate::grid::Position;
use crate::grid::Direction;
use crate::anchor::Anchor;

pub struct GridWalk<'a> {
    anchor: Anchor,
    steerings: &'a [Steering],
    has_entered_new: bool,
}

impl<'a> GridWalk<'a> {
    /// The number of pipes traversed by this walk.
    pub fn len_segments(&self) -> usize {
        self.steerings.len() * 2 + if self.has_entered_new { 1 } else { 0 }
    }

    /// The number of characters traversed by this walk.
    pub fn len(&self) -> usize {
        let len_seg = self.len_segments();
        (len_seg / 2) + (len_seg % 2)
    }

    pub fn iter_positions(&self, width: usize, height: usize) -> PositionIter<'a> {
        PositionIter::new(self.steerings, self.anchor, self.has_entered_new, width, height)
    }
}

pub struct PositionIter<'a> {
    steerings_iter: std::slice::Iter<'a, Steering>,
    has_entered_new: bool,

    curr_pos: Position,
    curr_dir: Direction,
}

impl<'a> PositionIter<'a> {
    pub fn new(steerings: &'a [Steering], anchor: Anchor, has_entered_new: bool, width: usize, height: usize) -> Self {
        let steerings_iter = steerings.iter();
        let curr_pos = anchor.position(width, height);
        let curr_dir = anchor.direction();

        Self {
            steerings_iter,
            has_entered_new,
            curr_pos,
            curr_dir,
        }
    }
}

impl<'a> Iterator for PositionIter<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        // See if there is another steering in the iterator.
        match self.steerings_iter.next() {
            None => {
                // Check if the next cell has been entered.
                if self.has_entered_new { Some(self.curr_pos) }
                else { None }
            },
            Some(steering) => {
                // Save the current position to return later, and update the position and direction.
                let pos = self.curr_pos;

                let next_dir = self.curr_dir.steer(*steering);
                self.curr_dir = next_dir;
                self.curr_pos = self.curr_pos.shift(next_dir, 1);

                Some(pos)
            }
        }
    }
}

impl<'a> std::iter::FusedIterator for PositionIter<'a> {}
