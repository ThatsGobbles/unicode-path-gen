use crate::grid::Direction;
use crate::grid::Position;
use crate::grid::Steering;
use crate::grid::PipeCell;

// pub enum Alignment {
//     Center,
//     Border(Direction),
// }

// pub struct Anchor(Position)

pub struct Course<'a> {
    steerings: &'a [Steering],
    has_tail: bool,
    has_entered: bool,
}

impl<'a> Course<'a> {
    pub fn len_segments(&self) -> usize {
        self.steerings.len() * 2 + self.has_entered as usize + self.has_tail as usize
    }
}

impl<'a> Default for Course<'a> {
    fn default() -> Self {
        Course {
            steerings: &[],
            has_tail: false,
            has_entered: false,
        }
    }
}

pub struct Walk<'a> {
    position: Position,
    heading: Direction,
    course: Course<'a>,
}

impl<'a> Walk<'a> {
    pub fn generate_cells(&self) -> Vec<(Position, PipeCell)> {
        let mut outputs = Vec::new();

        if self.course.has_tail {
            outputs.push((Position::from_raw(0, 0), PipeCell::default()));
        }

        outputs
    }
}

pub struct WalkIter<'a> {
    steerings_iter: std::slice::Iter<'a, Steering>,
    emit_tail: bool,
    emit_entrance: bool,

    curr_pos: Position,
    curr_dir: Direction,
}

impl<'a> WalkIter<'a> {
    pub fn new(walk: Walk<'a>) -> Self {
        let steerings_iter = walk.course.steerings.iter();
        let emit_tail = walk.course.has_tail;
        let emit_entrance = walk.course.has_entered;
        let curr_pos = walk.position;
        let curr_dir = walk.heading;

        Self {
            steerings_iter,
            emit_tail,
            emit_entrance,
            curr_pos,
            curr_dir,
        }
    }
}

impl<'a> Iterator for WalkIter<'a> {
    type Item = (Position, PipeCell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.emit_tail {
            self.emit_tail = false;

            // Calculate what the tail position and cell would be.
        }

        None
    }
}
