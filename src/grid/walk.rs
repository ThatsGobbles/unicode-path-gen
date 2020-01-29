use crate::grid::Direction;
use crate::grid::Position;
use crate::grid::Steering;

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

pub struct WalkIter<'a> {
    steerings_iter: std::slice::Iter<'a, Steering>,
    has_tail: bool,
    has_entered: bool,

    x_length: usize,
    y_length: usize,

    curr_pos: Position,
    curr_dir: Direction,
}

impl<'a> WalkIter<'a> {
    pub fn new(walk: Walk<'a>, x_length: usize, y_length: usize) -> Self {
        let steerings_iter = walk.course.steerings.iter();
        let has_tail = walk.course.has_tail;
        let has_entered = walk.course.has_entered;
        let curr_pos = walk.position;
        let curr_dir = walk.heading;

        Self {
            steerings_iter,
            has_tail,
            has_entered,
            x_length,
            y_length,
            curr_pos,
            curr_dir,
        }
    }
}
