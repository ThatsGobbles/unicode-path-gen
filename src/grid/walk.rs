use std::iter::FusedIterator;

use crate::grid::Direction;
use crate::grid::Position;
use crate::grid::Steering;
use crate::grid::Cell;

#[derive(Clone, Copy)]
pub struct Walk<'a> {
    start_position: Position,
    start_direction: Direction,

    steerings: &'a [Steering],
    starts_in_center: bool,
    closes_in_center: bool,
}

impl<'a> Walk<'a> {
    pub fn new(
        start_position: Position,
        start_direction: Direction,
        steerings: &'a [Steering],
        starts_in_center: bool,
        closes_in_center: bool,
        ) -> Self
    {
        Self {
            start_position,
            start_direction,
            steerings,
            starts_in_center,
            closes_in_center,
        }
    }

    pub fn len_segments(&self) -> usize {
        self.steerings.len() * 2 + self.closes_in_center as usize + self.starts_in_center as usize
    }

    pub fn iter(&self) -> WalkIter<'a> {
        WalkIter::new(self)
    }
}

pub struct WalkIter<'a> {
    steerings_iter: std::slice::Iter<'a, Steering>,
    starts_in_center: bool,
    closes_in_center: bool,

    curr_pos: Position,
    curr_dir: Direction,
}

impl<'a> WalkIter<'a> {
    pub fn new(walk: &Walk<'a>) -> Self {
        let steerings_iter = walk.steerings.iter();
        let starts_in_center = walk.starts_in_center;
        let closes_in_center = walk.closes_in_center;
        let curr_pos = walk.start_position;
        let curr_dir = walk.start_direction;

        Self {
            steerings_iter,
            starts_in_center,
            closes_in_center,
            curr_pos,
            curr_dir,
        }
    }
}

impl<'a> Iterator for WalkIter<'a> {
    type Item = (Position, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        // Check to see if a tail needs to be emitted.
        if self.starts_in_center {
            self.starts_in_center = false;

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
        else if self.closes_in_center {
            self.closes_in_center = false;

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
        let dummy_pos = Position::from_raw(0, 0);
        let dummy_dir = Direction::North;

        let empty_walk = Walk::new(dummy_pos, dummy_dir, &[], false, false);
        let mut iter = empty_walk.iter();
        assert_eq!(None, iter.next());

        let head_only_walk = Walk::new(dummy_pos, dummy_dir, &[], false, true);
        let mut iter = head_only_walk.iter();
        assert_eq!(Some((dummy_pos, Cell::from(dummy_dir.invert()))), iter.next());
        assert_eq!(None, iter.next());

        let tail_only_walk = Walk::new(dummy_pos, dummy_dir, &[], true, false);
        let mut iter = tail_only_walk.iter();
        assert_eq!(Some((dummy_pos, Cell::from(dummy_dir))), iter.next());
        assert_eq!(None, iter.next());

        let head_tail_only_walk = Walk::new(dummy_pos, dummy_dir, &[], true, true);
        let mut iter = head_tail_only_walk.iter();
        assert_eq!(Some((dummy_pos, Cell::from(dummy_dir.invert()))), iter.next());
        assert_eq!(Some((dummy_pos.shift(dummy_dir, 1), Cell::from(dummy_dir))), iter.next());
        assert_eq!(None, iter.next());

        let steerings = &[
            Steering::Right,
            Steering::Left,
            Steering::Straight,
            Steering::Left,
        ];
    }

    #[test]
    fn next() {
        let steerings = &[
            Steering::Left,
            Steering::Straight,
            Steering::Right,
            Steering::Straight,
            Steering::Straight,
            Steering::Right,
            Steering::Left,
        ];

        let walk = Walk {
            start_position: Position::from_raw(17, 17),
            start_direction: Direction::East,
            steerings: steerings,
            // starts_in_center: true,
            starts_in_center: true,
            closes_in_center: true,
        };

        for (pos, cell) in walk.iter() {
            println!("({}, '{}')", pos, cell.char());
        }
    }
}
