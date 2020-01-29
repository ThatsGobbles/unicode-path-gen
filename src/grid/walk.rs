use crate::grid::Direction;
use crate::grid::Position;
use crate::grid::Steering;
use crate::grid::Pipe;
use crate::grid::Cell;

// pub enum Alignment {
//     Center,
//     Border(Direction),
// }

// pub struct Anchor(Position)

pub struct Course<'a> {
    steerings: &'a [Steering],
    has_tail: bool,
    has_head: bool,
}

impl<'a> Course<'a> {
    pub fn len_segments(&self) -> usize {
        self.steerings.len() * 2 + self.has_head as usize + self.has_tail as usize
    }
}

impl<'a> Default for Course<'a> {
    fn default() -> Self {
        Course {
            steerings: &[],
            has_tail: false,
            has_head: false,
        }
    }
}

pub struct Walk<'a> {
    position: Position,
    heading: Direction,
    course: Course<'a>,
}

pub struct WalkIter<'a> {
    steerings_iter: std::slice::Iter<'a, Steering>,
    emit_tail: bool,
    emit_head: bool,

    curr_pos: Position,
    curr_dir: Direction,
}

impl<'a> WalkIter<'a> {
    pub fn new(walk: Walk<'a>) -> Self {
        let steerings_iter = walk.course.steerings.iter();
        let emit_tail = walk.course.has_tail;
        let emit_head = walk.course.has_head;
        let curr_pos = walk.position;
        let curr_dir = walk.heading;

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

            // Calculate what the tail position would be.
            // This would be the current position, shifted one spot in the
            // inverse of the current direction.
            let pos = self.curr_pos.shift(self.curr_dir.invert(), 1);

            let cell = Cell::default().with_dir(self.curr_dir, Pipe::Empty);

            Some((pos, cell))
        }
        // Try and see if there are any `Steering`s available.
        else if let Some(steering) = self.steerings_iter.next() {
            // A `Steering` adjusts the current position and direction.
            let next_dir = self.curr_dir.steer(*steering);
            let next_pos = self.curr_pos.shift(next_dir, 1);

            let pos = self.curr_pos;

            // Each `Steering` represents an entrance AND exit from a cell.
            let cell =
                Cell::default()
                .with_dir(self.curr_dir.invert(), Pipe::Empty)
                .with_dir(next_dir.invert(), Pipe::Empty)
            ;

            self.curr_dir = next_dir;
            self.curr_pos = next_pos;

            Some((pos, cell))
        }
        // Check to see if a head needs to be emitted.
        else if self.emit_head {
            self.emit_head = false;

            // The head position is just the current position.
            let pos = self.curr_pos;

            let cell = Cell::default().with_dir(self.curr_dir.invert(), Pipe::Empty);

            Some((pos, cell))
        }
        // Nothing else to do.
        else {
            None
        }
    }
}
