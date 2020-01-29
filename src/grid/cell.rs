use crate::grid::Direction;

const XN_XS_XW_XE: char = ' ';
const XN_XS_XW_EE: char = '╶';
const XN_XS_XW_FE: char = '╺';
const XN_XS_EW_XE: char = '╴';
const XN_XS_EW_EE: char = '─';
const XN_XS_EW_FE: char = '╼';
const XN_XS_FW_XE: char = '╸';
const XN_XS_FW_EE: char = '╾';
const XN_XS_FW_FE: char = '━';
const XN_ES_XW_XE: char = '╷';
const XN_ES_XW_EE: char = '┌';
const XN_ES_XW_FE: char = '┍';
const XN_ES_EW_XE: char = '┐';
const XN_ES_EW_EE: char = '┬';
const XN_ES_EW_FE: char = '┮';
const XN_ES_FW_XE: char = '┑';
const XN_ES_FW_EE: char = '┭';
const XN_ES_FW_FE: char = '┯';
const XN_FS_XW_XE: char = '╻';
const XN_FS_XW_EE: char = '┎';
const XN_FS_XW_FE: char = '┏';
const XN_FS_EW_XE: char = '┒';
const XN_FS_EW_EE: char = '┰';
const XN_FS_EW_FE: char = '┲';
const XN_FS_FW_XE: char = '┓';
const XN_FS_FW_EE: char = '┱';
const XN_FS_FW_FE: char = '┳';
const EN_XS_XW_XE: char = '╵';
const EN_XS_XW_EE: char = '└';
const EN_XS_XW_FE: char = '┕';
const EN_XS_EW_XE: char = '┘';
const EN_XS_EW_EE: char = '┴';
const EN_XS_EW_FE: char = '┶';
const EN_XS_FW_XE: char = '┙';
const EN_XS_FW_EE: char = '┵';
const EN_XS_FW_FE: char = '┷';
const EN_ES_XW_XE: char = '│';
const EN_ES_XW_EE: char = '├';
const EN_ES_XW_FE: char = '┝';
const EN_ES_EW_XE: char = '┤';
const EN_ES_EW_EE: char = '┼';
const EN_ES_EW_FE: char = '┾';
const EN_ES_FW_XE: char = '┥';
const EN_ES_FW_EE: char = '┽';
const EN_ES_FW_FE: char = '┿';
const EN_FS_XW_XE: char = '╽';
const EN_FS_XW_EE: char = '┟';
const EN_FS_XW_FE: char = '┢';
const EN_FS_EW_XE: char = '┧';
const EN_FS_EW_EE: char = '╁';
const EN_FS_EW_FE: char = '╆';
const EN_FS_FW_XE: char = '┪';
const EN_FS_FW_EE: char = '╅';
const EN_FS_FW_FE: char = '╈';
const FN_XS_XW_XE: char = '╹';
const FN_XS_XW_EE: char = '┖';
const FN_XS_XW_FE: char = '┗';
const FN_XS_EW_XE: char = '┚';
const FN_XS_EW_EE: char = '┸';
const FN_XS_EW_FE: char = '┺';
const FN_XS_FW_XE: char = '┛';
const FN_XS_FW_EE: char = '┹';
const FN_XS_FW_FE: char = '┻';
const FN_ES_XW_XE: char = '╿';
const FN_ES_XW_EE: char = '┞';
const FN_ES_XW_FE: char = '┡';
const FN_ES_EW_XE: char = '┦';
const FN_ES_EW_EE: char = '╀';
const FN_ES_EW_FE: char = '╄';
const FN_ES_FW_XE: char = '┩';
const FN_ES_FW_EE: char = '╃';
const FN_ES_FW_FE: char = '╇';
const FN_FS_XW_XE: char = '┃';
const FN_FS_XW_EE: char = '┠';
const FN_FS_XW_FE: char = '┣';
const FN_FS_EW_XE: char = '┨';
const FN_FS_EW_EE: char = '╂';
const FN_FS_EW_FE: char = '╊';
const FN_FS_FW_XE: char = '┫';
const FN_FS_FW_EE: char = '╉';
const FN_FS_FW_FE: char = '╋';

#[derive(Copy, Clone)]
pub enum Pipe {
    None,
    Empty,
    Full,
}

#[derive(Copy, Clone)]
pub struct Cell {
    n: Pipe,
    s: Pipe,
    w: Pipe,
    e: Pipe,
}

impl Default for Cell {
    fn default() -> Self {
        Self { n: Pipe::None, s: Pipe::None, w: Pipe::None, e: Pipe::None, }
    }
}

impl Cell {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn with_dir(&self, dir: Direction, pipe: Pipe) -> Self {
        let mut cell = *self;

        match dir {
            Direction::North => cell.n = pipe,
            Direction::South => cell.s = pipe,
            Direction::West => cell.w = pipe,
            Direction::East => cell.e = pipe,
        };

        cell
    }

    pub fn char(&self) -> char {
        match (self.n, self.s, self.w, self.e) {
            ( Pipe::None,  Pipe::None,  Pipe::None,  Pipe::None,) => XN_XS_XW_XE,
            ( Pipe::None,  Pipe::None,  Pipe::None, Pipe::Empty,) => XN_XS_XW_EE,
            ( Pipe::None,  Pipe::None,  Pipe::None,  Pipe::Full,) => XN_XS_XW_FE,
            ( Pipe::None,  Pipe::None, Pipe::Empty,  Pipe::None,) => XN_XS_EW_XE,
            ( Pipe::None,  Pipe::None, Pipe::Empty, Pipe::Empty,) => XN_XS_EW_EE,
            ( Pipe::None,  Pipe::None, Pipe::Empty,  Pipe::Full,) => XN_XS_EW_FE,
            ( Pipe::None,  Pipe::None,  Pipe::Full,  Pipe::None,) => XN_XS_FW_XE,
            ( Pipe::None,  Pipe::None,  Pipe::Full, Pipe::Empty,) => XN_XS_FW_EE,
            ( Pipe::None,  Pipe::None,  Pipe::Full,  Pipe::Full,) => XN_XS_FW_FE,
            ( Pipe::None, Pipe::Empty,  Pipe::None,  Pipe::None,) => XN_ES_XW_XE,
            ( Pipe::None, Pipe::Empty,  Pipe::None, Pipe::Empty,) => XN_ES_XW_EE,
            ( Pipe::None, Pipe::Empty,  Pipe::None,  Pipe::Full,) => XN_ES_XW_FE,
            ( Pipe::None, Pipe::Empty, Pipe::Empty,  Pipe::None,) => XN_ES_EW_XE,
            ( Pipe::None, Pipe::Empty, Pipe::Empty, Pipe::Empty,) => XN_ES_EW_EE,
            ( Pipe::None, Pipe::Empty, Pipe::Empty,  Pipe::Full,) => XN_ES_EW_FE,
            ( Pipe::None, Pipe::Empty,  Pipe::Full,  Pipe::None,) => XN_ES_FW_XE,
            ( Pipe::None, Pipe::Empty,  Pipe::Full, Pipe::Empty,) => XN_ES_FW_EE,
            ( Pipe::None, Pipe::Empty,  Pipe::Full,  Pipe::Full,) => XN_ES_FW_FE,
            ( Pipe::None,  Pipe::Full,  Pipe::None,  Pipe::None,) => XN_FS_XW_XE,
            ( Pipe::None,  Pipe::Full,  Pipe::None, Pipe::Empty,) => XN_FS_XW_EE,
            ( Pipe::None,  Pipe::Full,  Pipe::None,  Pipe::Full,) => XN_FS_XW_FE,
            ( Pipe::None,  Pipe::Full, Pipe::Empty,  Pipe::None,) => XN_FS_EW_XE,
            ( Pipe::None,  Pipe::Full, Pipe::Empty, Pipe::Empty,) => XN_FS_EW_EE,
            ( Pipe::None,  Pipe::Full, Pipe::Empty,  Pipe::Full,) => XN_FS_EW_FE,
            ( Pipe::None,  Pipe::Full,  Pipe::Full,  Pipe::None,) => XN_FS_FW_XE,
            ( Pipe::None,  Pipe::Full,  Pipe::Full, Pipe::Empty,) => XN_FS_FW_EE,
            ( Pipe::None,  Pipe::Full,  Pipe::Full,  Pipe::Full,) => XN_FS_FW_FE,
            (Pipe::Empty,  Pipe::None,  Pipe::None,  Pipe::None,) => EN_XS_XW_XE,
            (Pipe::Empty,  Pipe::None,  Pipe::None, Pipe::Empty,) => EN_XS_XW_EE,
            (Pipe::Empty,  Pipe::None,  Pipe::None,  Pipe::Full,) => EN_XS_XW_FE,
            (Pipe::Empty,  Pipe::None, Pipe::Empty,  Pipe::None,) => EN_XS_EW_XE,
            (Pipe::Empty,  Pipe::None, Pipe::Empty, Pipe::Empty,) => EN_XS_EW_EE,
            (Pipe::Empty,  Pipe::None, Pipe::Empty,  Pipe::Full,) => EN_XS_EW_FE,
            (Pipe::Empty,  Pipe::None,  Pipe::Full,  Pipe::None,) => EN_XS_FW_XE,
            (Pipe::Empty,  Pipe::None,  Pipe::Full, Pipe::Empty,) => EN_XS_FW_EE,
            (Pipe::Empty,  Pipe::None,  Pipe::Full,  Pipe::Full,) => EN_XS_FW_FE,
            (Pipe::Empty, Pipe::Empty,  Pipe::None,  Pipe::None,) => EN_ES_XW_XE,
            (Pipe::Empty, Pipe::Empty,  Pipe::None, Pipe::Empty,) => EN_ES_XW_EE,
            (Pipe::Empty, Pipe::Empty,  Pipe::None,  Pipe::Full,) => EN_ES_XW_FE,
            (Pipe::Empty, Pipe::Empty, Pipe::Empty,  Pipe::None,) => EN_ES_EW_XE,
            (Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty,) => EN_ES_EW_EE,
            (Pipe::Empty, Pipe::Empty, Pipe::Empty,  Pipe::Full,) => EN_ES_EW_FE,
            (Pipe::Empty, Pipe::Empty,  Pipe::Full,  Pipe::None,) => EN_ES_FW_XE,
            (Pipe::Empty, Pipe::Empty,  Pipe::Full, Pipe::Empty,) => EN_ES_FW_EE,
            (Pipe::Empty, Pipe::Empty,  Pipe::Full,  Pipe::Full,) => EN_ES_FW_FE,
            (Pipe::Empty,  Pipe::Full,  Pipe::None,  Pipe::None,) => EN_FS_XW_XE,
            (Pipe::Empty,  Pipe::Full,  Pipe::None, Pipe::Empty,) => EN_FS_XW_EE,
            (Pipe::Empty,  Pipe::Full,  Pipe::None,  Pipe::Full,) => EN_FS_XW_FE,
            (Pipe::Empty,  Pipe::Full, Pipe::Empty,  Pipe::None,) => EN_FS_EW_XE,
            (Pipe::Empty,  Pipe::Full, Pipe::Empty, Pipe::Empty,) => EN_FS_EW_EE,
            (Pipe::Empty,  Pipe::Full, Pipe::Empty,  Pipe::Full,) => EN_FS_EW_FE,
            (Pipe::Empty,  Pipe::Full,  Pipe::Full,  Pipe::None,) => EN_FS_FW_XE,
            (Pipe::Empty,  Pipe::Full,  Pipe::Full, Pipe::Empty,) => EN_FS_FW_EE,
            (Pipe::Empty,  Pipe::Full,  Pipe::Full,  Pipe::Full,) => EN_FS_FW_FE,
            ( Pipe::Full,  Pipe::None,  Pipe::None,  Pipe::None,) => FN_XS_XW_XE,
            ( Pipe::Full,  Pipe::None,  Pipe::None, Pipe::Empty,) => FN_XS_XW_EE,
            ( Pipe::Full,  Pipe::None,  Pipe::None,  Pipe::Full,) => FN_XS_XW_FE,
            ( Pipe::Full,  Pipe::None, Pipe::Empty,  Pipe::None,) => FN_XS_EW_XE,
            ( Pipe::Full,  Pipe::None, Pipe::Empty, Pipe::Empty,) => FN_XS_EW_EE,
            ( Pipe::Full,  Pipe::None, Pipe::Empty,  Pipe::Full,) => FN_XS_EW_FE,
            ( Pipe::Full,  Pipe::None,  Pipe::Full,  Pipe::None,) => FN_XS_FW_XE,
            ( Pipe::Full,  Pipe::None,  Pipe::Full, Pipe::Empty,) => FN_XS_FW_EE,
            ( Pipe::Full,  Pipe::None,  Pipe::Full,  Pipe::Full,) => FN_XS_FW_FE,
            ( Pipe::Full, Pipe::Empty,  Pipe::None,  Pipe::None,) => FN_ES_XW_XE,
            ( Pipe::Full, Pipe::Empty,  Pipe::None, Pipe::Empty,) => FN_ES_XW_EE,
            ( Pipe::Full, Pipe::Empty,  Pipe::None,  Pipe::Full,) => FN_ES_XW_FE,
            ( Pipe::Full, Pipe::Empty, Pipe::Empty,  Pipe::None,) => FN_ES_EW_XE,
            ( Pipe::Full, Pipe::Empty, Pipe::Empty, Pipe::Empty,) => FN_ES_EW_EE,
            ( Pipe::Full, Pipe::Empty, Pipe::Empty,  Pipe::Full,) => FN_ES_EW_FE,
            ( Pipe::Full, Pipe::Empty,  Pipe::Full,  Pipe::None,) => FN_ES_FW_XE,
            ( Pipe::Full, Pipe::Empty,  Pipe::Full, Pipe::Empty,) => FN_ES_FW_EE,
            ( Pipe::Full, Pipe::Empty,  Pipe::Full,  Pipe::Full,) => FN_ES_FW_FE,
            ( Pipe::Full,  Pipe::Full,  Pipe::None,  Pipe::None,) => FN_FS_XW_XE,
            ( Pipe::Full,  Pipe::Full,  Pipe::None, Pipe::Empty,) => FN_FS_XW_EE,
            ( Pipe::Full,  Pipe::Full,  Pipe::None,  Pipe::Full,) => FN_FS_XW_FE,
            ( Pipe::Full,  Pipe::Full, Pipe::Empty,  Pipe::None,) => FN_FS_EW_XE,
            ( Pipe::Full,  Pipe::Full, Pipe::Empty, Pipe::Empty,) => FN_FS_EW_EE,
            ( Pipe::Full,  Pipe::Full, Pipe::Empty,  Pipe::Full,) => FN_FS_EW_FE,
            ( Pipe::Full,  Pipe::Full,  Pipe::Full,  Pipe::None,) => FN_FS_FW_XE,
            ( Pipe::Full,  Pipe::Full,  Pipe::Full, Pipe::Empty,) => FN_FS_FW_EE,
            ( Pipe::Full,  Pipe::Full,  Pipe::Full,  Pipe::Full,) => FN_FS_FW_FE,
        }
    }
}
