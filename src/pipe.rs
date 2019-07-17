use crate::coordinate::Coordinate;

const XU_XD_XL_XR: char = ' ';
const XU_XD_XL_ER: char = '╶';
const XU_XD_XL_FR: char = '╺';
const XU_XD_EL_XR: char = '╴';
const XU_XD_EL_ER: char = '─';
const XU_XD_EL_FR: char = '╼';
const XU_XD_FL_XR: char = '╸';
const XU_XD_FL_ER: char = '╾';
const XU_XD_FL_FR: char = '━';
const XU_ED_XL_XR: char = '╷';
const XU_ED_XL_ER: char = '┌';
const XU_ED_XL_FR: char = '┍';
const XU_ED_EL_XR: char = '┐';
const XU_ED_EL_ER: char = '┬';
const XU_ED_EL_FR: char = '┮';
const XU_ED_FL_XR: char = '┑';
const XU_ED_FL_ER: char = '┭';
const XU_ED_FL_FR: char = '┯';
const XU_FD_XL_XR: char = '╻';
const XU_FD_XL_ER: char = '┎';
const XU_FD_XL_FR: char = '┏';
const XU_FD_EL_XR: char = '┒';
const XU_FD_EL_ER: char = '┰';
const XU_FD_EL_FR: char = '┲';
const XU_FD_FL_XR: char = '┓';
const XU_FD_FL_ER: char = '┱';
const XU_FD_FL_FR: char = '┳';
const EU_XD_XL_XR: char = '╵';
const EU_XD_XL_ER: char = '└';
const EU_XD_XL_FR: char = '┕';
const EU_XD_EL_XR: char = '┘';
const EU_XD_EL_ER: char = '┴';
const EU_XD_EL_FR: char = '┶';
const EU_XD_FL_XR: char = '┙';
const EU_XD_FL_ER: char = '┵';
const EU_XD_FL_FR: char = '┷';
const EU_ED_XL_XR: char = '│';
const EU_ED_XL_ER: char = '├';
const EU_ED_XL_FR: char = '┝';
const EU_ED_EL_XR: char = '┤';
const EU_ED_EL_ER: char = '┼';
const EU_ED_EL_FR: char = '┾';
const EU_ED_FL_XR: char = '┥';
const EU_ED_FL_ER: char = '┽';
const EU_ED_FL_FR: char = '┿';
const EU_FD_XL_XR: char = '╽';
const EU_FD_XL_ER: char = '┟';
const EU_FD_XL_FR: char = '┢';
const EU_FD_EL_XR: char = '┧';
const EU_FD_EL_ER: char = '╁';
const EU_FD_EL_FR: char = '╆';
const EU_FD_FL_XR: char = '┪';
const EU_FD_FL_ER: char = '╅';
const EU_FD_FL_FR: char = '╈';
const FU_XD_XL_XR: char = '╹';
const FU_XD_XL_ER: char = '┖';
const FU_XD_XL_FR: char = '┗';
const FU_XD_EL_XR: char = '┚';
const FU_XD_EL_ER: char = '┸';
const FU_XD_EL_FR: char = '┺';
const FU_XD_FL_XR: char = '┛';
const FU_XD_FL_ER: char = '┹';
const FU_XD_FL_FR: char = '┻';
const FU_ED_XL_XR: char = '╿';
const FU_ED_XL_ER: char = '┞';
const FU_ED_XL_FR: char = '┡';
const FU_ED_EL_XR: char = '┦';
const FU_ED_EL_ER: char = '╀';
const FU_ED_EL_FR: char = '╄';
const FU_ED_FL_XR: char = '┩';
const FU_ED_FL_ER: char = '╃';
const FU_ED_FL_FR: char = '╇';
const FU_FD_XL_XR: char = '┃';
const FU_FD_XL_ER: char = '┠';
const FU_FD_XL_FR: char = '┣';
const FU_FD_EL_XR: char = '┨';
const FU_FD_EL_ER: char = '╂';
const FU_FD_EL_FR: char = '╊';
const FU_FD_FL_XR: char = '┫';
const FU_FD_FL_ER: char = '╉';
const FU_FD_FL_FR: char = '╋';

#[derive(Copy, Clone)]
pub enum Steering {
    Straight,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn steer(&self, s: Steering) -> Self {
        match s {
            Steering::Straight => *self,
            Steering::Left => {
                match self {
                    &Direction::Up => Direction::Left,
                    &Direction::Down => Direction::Right,
                    &Direction::Left => Direction::Down,
                    &Direction::Right => Direction::Up,
                }
            },
            Steering::Right => {
                match self {
                    &Direction::Up => Direction::Right,
                    &Direction::Down => Direction::Left,
                    &Direction::Left => Direction::Up,
                    &Direction::Right => Direction::Down,
                }
            },
        }
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    x: Coordinate,
    y: Coordinate,
}

impl From<(Coordinate, Coordinate)> for Position {
    fn from((x, y): (Coordinate, Coordinate)) -> Self {
        Self { x, y, }
    }
}

impl From<(usize, usize)> for Position {
    fn from((nx, ny): (usize, usize)) -> Self {
        let x = nx.into();
        let y = ny.into();
        Self { x, y, }
    }
}

impl Position {
    pub fn shift(&self, direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::Up       => (self.x, self.y.sub(1)),
            Direction::Down     => (self.x, self.y.add(1)),
            Direction::Left     => (self.x.sub(1), self.y),
            Direction::Right    => (self.x.add(1), self.y),
        };

        Self { x, y, }
    }
}

#[derive(Copy, Clone)]
pub enum Pipe {
    Empty,
    Full,
}

#[derive(Copy, Clone)]
pub struct PipeCell {
    u: Option<Pipe>,
    d: Option<Pipe>,
    l: Option<Pipe>,
    r: Option<Pipe>,
}

impl Default for PipeCell {
    fn default() -> Self {
        Self { u: None, d: None, l: None, r: None, }
    }
}

impl PipeCell {
    pub fn clear(&mut self) {
        self.u = None;
        self.d = None;
        self.l = None;
        self.r = None;
    }

    pub fn lay_down(&mut self, dir: Direction, pipe: Pipe) -> Option<Pipe> {
        match dir {
            Direction::Up => self.u.replace(pipe),
            Direction::Down => self.d.replace(pipe),
            Direction::Left => self.l.replace(pipe),
            Direction::Right => self.r.replace(pipe),
        }
    }

    pub fn dig_up(&mut self, dir: Direction) -> Option<Pipe> {
        match dir {
            Direction::Up => self.u.take(),
            Direction::Down => self.d.take(),
            Direction::Left => self.l.take(),
            Direction::Right => self.r.take(),
        }
    }

    pub fn char(&self) -> char {
        match (self.u, self.d, self.l, self.r) {
            (             None,              None,              None,              None,) => XU_XD_XL_XR,
            (             None,              None,              None, Some(Pipe::Empty),) => XU_XD_XL_ER,
            (             None,              None,              None,  Some(Pipe::Full),) => XU_XD_XL_FR,
            (             None,              None, Some(Pipe::Empty),              None,) => XU_XD_EL_XR,
            (             None,              None, Some(Pipe::Empty), Some(Pipe::Empty),) => XU_XD_EL_ER,
            (             None,              None, Some(Pipe::Empty),  Some(Pipe::Full),) => XU_XD_EL_FR,
            (             None,              None,  Some(Pipe::Full),              None,) => XU_XD_FL_XR,
            (             None,              None,  Some(Pipe::Full), Some(Pipe::Empty),) => XU_XD_FL_ER,
            (             None,              None,  Some(Pipe::Full),  Some(Pipe::Full),) => XU_XD_FL_FR,
            (             None, Some(Pipe::Empty),              None,              None,) => XU_ED_XL_XR,
            (             None, Some(Pipe::Empty),              None, Some(Pipe::Empty),) => XU_ED_XL_ER,
            (             None, Some(Pipe::Empty),              None,  Some(Pipe::Full),) => XU_ED_XL_FR,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty),              None,) => XU_ED_EL_XR,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),) => XU_ED_EL_ER,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),) => XU_ED_EL_FR,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full),              None,) => XU_ED_FL_XR,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),) => XU_ED_FL_ER,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),) => XU_ED_FL_FR,
            (             None,  Some(Pipe::Full),              None,              None,) => XU_FD_XL_XR,
            (             None,  Some(Pipe::Full),              None, Some(Pipe::Empty),) => XU_FD_XL_ER,
            (             None,  Some(Pipe::Full),              None,  Some(Pipe::Full),) => XU_FD_XL_FR,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty),              None,) => XU_FD_EL_XR,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),) => XU_FD_EL_ER,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),) => XU_FD_EL_FR,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full),              None,) => XU_FD_FL_XR,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),) => XU_FD_FL_ER,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),) => XU_FD_FL_FR,
            (Some(Pipe::Empty),              None,              None,              None,) => EU_XD_XL_XR,
            (Some(Pipe::Empty),              None,              None, Some(Pipe::Empty),) => EU_XD_XL_ER,
            (Some(Pipe::Empty),              None,              None,  Some(Pipe::Full),) => EU_XD_XL_FR,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty),              None,) => EU_XD_EL_XR,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty), Some(Pipe::Empty),) => EU_XD_EL_ER,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty),  Some(Pipe::Full),) => EU_XD_EL_FR,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full),              None,) => EU_XD_FL_XR,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full), Some(Pipe::Empty),) => EU_XD_FL_ER,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full),  Some(Pipe::Full),) => EU_XD_FL_FR,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None,              None,) => EU_ED_XL_XR,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None, Some(Pipe::Empty),) => EU_ED_XL_ER,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None,  Some(Pipe::Full),) => EU_ED_XL_FR,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),              None,) => EU_ED_EL_XR,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),) => EU_ED_EL_ER,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),) => EU_ED_EL_FR,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),              None,) => EU_ED_FL_XR,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),) => EU_ED_FL_ER,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),) => EU_ED_FL_FR,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None,              None,) => EU_FD_XL_XR,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None, Some(Pipe::Empty),) => EU_FD_XL_ER,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None,  Some(Pipe::Full),) => EU_FD_XL_FR,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),              None,) => EU_FD_EL_XR,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),) => EU_FD_EL_ER,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),) => EU_FD_EL_FR,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),              None,) => EU_FD_FL_XR,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),) => EU_FD_FL_ER,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),) => EU_FD_FL_FR,
            ( Some(Pipe::Full),              None,              None,              None,) => FU_XD_XL_XR,
            ( Some(Pipe::Full),              None,              None, Some(Pipe::Empty),) => FU_XD_XL_ER,
            ( Some(Pipe::Full),              None,              None,  Some(Pipe::Full),) => FU_XD_XL_FR,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty),              None,) => FU_XD_EL_XR,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty), Some(Pipe::Empty),) => FU_XD_EL_ER,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty),  Some(Pipe::Full),) => FU_XD_EL_FR,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full),              None,) => FU_XD_FL_XR,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full), Some(Pipe::Empty),) => FU_XD_FL_ER,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full),  Some(Pipe::Full),) => FU_XD_FL_FR,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None,              None,) => FU_ED_XL_XR,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None, Some(Pipe::Empty),) => FU_ED_XL_ER,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None,  Some(Pipe::Full),) => FU_ED_XL_FR,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),              None,) => FU_ED_EL_XR,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),) => FU_ED_EL_ER,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),) => FU_ED_EL_FR,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),              None,) => FU_ED_FL_XR,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),) => FU_ED_FL_ER,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),) => FU_ED_FL_FR,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None,              None,) => FU_FD_XL_XR,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None, Some(Pipe::Empty),) => FU_FD_XL_ER,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None,  Some(Pipe::Full),) => FU_FD_XL_FR,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),              None,) => FU_FD_EL_XR,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),) => FU_FD_EL_ER,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),) => FU_FD_EL_FR,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),              None,) => FU_FD_FL_XR,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),) => FU_FD_FL_ER,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),) => FU_FD_FL_FR,
        }
    }
}

pub struct PipeGrid {
    grid: Vec<PipeCell>,
    width: usize,
    height: usize,
}

impl PipeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![PipeCell::default(); width * height];
        Self { grid, width, height, }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&PipeCell> {
        if x < self.width() && y < self.height() {
            self.grid.get(x * y)
        }
        else { None }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut PipeCell> {
        if x < self.width() && y < self.height() {
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

#[derive(Copy, Clone)]
pub enum Anchor {
    North(Coordinate),
    South(Coordinate),
    West(Coordinate),
    East(Coordinate),
}

impl Anchor {
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        match self {
            &Anchor::North(Coordinate::Pos(x)) => x < width,
            &Anchor::South(Coordinate::Pos(x)) => x < width,
            &Anchor::West(Coordinate::Pos(y)) => y < height,
            &Anchor::East(Coordinate::Pos(y)) => y < height,
            _ => false,
        }
    }

    pub fn direction(&self) -> Direction {
        match self {
            &Anchor::North(..) => Direction::Down,
            &Anchor::South(..) => Direction::Up,
            &Anchor::West(..) => Direction::Right,
            &Anchor::East(..) => Direction::Left,
        }
    }

    pub fn position(&self, width: usize, height: usize) -> Position {
        match self {
            &Anchor::North(x) => (x.into(), 0.into()),
            &Anchor::South(x) => (x.into(), Coordinate::from(height).sub(1)),
            &Anchor::West(y) => (0.into(), y.into()),
            &Anchor::East(y) => (Coordinate::from(width).sub(1), y.into()),
        }.into()
    }
}

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

    pub fn iter_positions(&self, width: usize, height: usize) -> GridWalkCoordinates<'a> {
        GridWalkCoordinates::new(self.steerings, self.anchor, self.has_entered_new, width, height)
    }
}

pub struct GridWalkCoordinates<'a> {
    steerings_iter: std::slice::Iter<'a, Steering>,
    has_entered_new: bool,

    curr_pos: Position,
    curr_dir: Direction,
}

impl<'a> GridWalkCoordinates<'a> {
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

impl<'a> Iterator for GridWalkCoordinates<'a> {
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
                self.curr_pos = self.curr_pos.shift(next_dir);

                Some(pos)
            }
        }
    }
}

impl<'a> std::iter::FusedIterator for GridWalkCoordinates<'a> {}
