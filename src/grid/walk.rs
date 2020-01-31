use std::iter::FusedIterator;

use crate::grid::Direction;
use crate::grid::Position;
use crate::grid::Steering;
use crate::grid::Cell;
use crate::grid::Anchor;
use crate::grid::Course;

#[derive(Clone, Copy)]
pub struct Walk<'a> {
    anchor: Anchor,
    course: Course<'a>,
}

impl<'a> Walk<'a> {
    pub fn new(anchor: Anchor, course: Course<'a>) -> Self {
        Self {
            anchor,
            course,
        }
    }

    pub fn has_tail(&self) -> bool {
        self.anchor.is_centered()
    }

    pub fn has_head(&self) -> bool {
        self.course.has_head()
    }

    pub fn len_segments(&self) -> usize {
        self.anchor.len_segments() + self.course.len_segments()
    }

    pub fn iter(&self) -> WalkIter<'a> {
        WalkIter::new(self)
    }

    pub fn start_position(&self) -> Position {
        self.anchor.position()
    }

    pub fn start_direction(&self) -> Direction {
        self.anchor.heading()
    }
}

pub struct WalkIter<'a> {
    steerings_iter: std::slice::Iter<'a, Steering>,
    emit_tail: bool,
    emit_head: bool,

    curr_pos: Position,
    curr_dir: Direction,
}

impl<'a> WalkIter<'a> {
    pub fn new(walk: &Walk<'a>) -> Self {
        let steerings_iter = walk.course.iter();
        let emit_tail = walk.has_tail();
        let emit_head = walk.has_head();
        let curr_pos = walk.start_position();
        let curr_dir = walk.start_direction();

        Self {
            steerings_iter,
            emit_tail,
            emit_head,
            curr_pos,
            curr_dir,
        }
    }
}

impl<'a> Iterator for WalkIter<'a> {
    type Item = (Position, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        // Check to see if a tail needs to be emitted.
        if self.emit_tail {
            self.emit_tail = false;

            let next_pos = self.curr_pos.shift(self.curr_dir, 1);

            // The tail position is just the current position.
            let pos = self.curr_pos;
            let cell = Cell::from(self.curr_dir);

            self.curr_pos = next_pos;

            Some((pos, cell))
        }
        // Try and see if there are any `Steering`s available.
        else if let Some(steering) = self.steerings_iter.next() {
            // A `Steering` adjusts the current position and direction.
            let next_dir = self.curr_dir.steer(*steering);
            let next_pos = self.curr_pos.shift(next_dir, 1);

            let pos = self.curr_pos;

            // Each `Steering` represents an entrance AND exit from a cell.
            let cell = Cell::from(self.curr_dir.invert()) | Cell::from(next_dir);

            self.curr_dir = next_dir;
            self.curr_pos = next_pos;

            Some((pos, cell))
        }
        // Check to see if a head needs to be emitted.
        else if self.emit_head {
            self.emit_head = false;

            // The head position is just the current position.
            let pos = self.curr_pos;

            let cell = Cell::from(self.curr_dir.invert());

            Some((pos, cell))
        }
        // Nothing else to do.
        else {
            None
        }
    }
}

impl<'a> FusedIterator for WalkIter<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invariants() {
        let sample_pos = Position::from_raw(0, 0);
        let sample_dir = Direction::North;
        let sample_pos_shifted = sample_pos.shift(sample_dir, 1);

        let border_anchor = Anchor::new(sample_pos, sample_dir, false);
        let center_anchor = Anchor::new(sample_pos, sample_dir, true);
        let empty_course = Course::empty();
        let head_only_course = Course::new(&[], true);

        let walk = Walk::new(border_anchor, empty_course);
        let mut iter = walk.iter();
        assert_eq!(None, iter.next());

        let walk = Walk::new(center_anchor, empty_course);
        let mut iter = walk.iter();
        assert_eq!(Some((sample_pos, Cell::from(sample_dir))), iter.next());
        assert_eq!(None, iter.next());

        let walk = Walk::new(border_anchor, head_only_course);
        let mut iter = walk.iter();
        assert_eq!(Some((sample_pos, Cell::from(sample_dir.invert()))), iter.next());
        assert_eq!(None, iter.next());

        let walk = Walk::new(center_anchor, head_only_course);
        let mut iter = walk.iter();
        assert_eq!(Some((sample_pos, Cell::from(sample_dir))), iter.next());
        assert_eq!(Some((sample_pos_shifted, Cell::from(sample_dir.invert()))), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn next() {
        // let steerings = &[
        //     Steering::Left,
        //     Steering::Straight,
        //     Steering::Right,
        //     Steering::Straight,
        //     Steering::Straight,
        //     Steering::Right,
        //     Steering::Left,
        // ];

        // let walk = Walk {
        //     start_position: Position::from_raw(17, 17),
        //     start_direction: Direction::East,
        //     steerings: steerings,
        //     // emit_tail: true,
        //     emit_tail: true,
        //     emit_head: true,
        // };

        // for (pos, cell) in walk.iter() {
        //     println!("({}, '{}')", pos, cell.char());
        // }
    }
}
