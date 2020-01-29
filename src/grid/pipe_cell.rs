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
    Empty,
    Full,
}

#[derive(Copy, Clone)]
pub struct PipeCell {
    n: Option<Pipe>,
    s: Option<Pipe>,
    w: Option<Pipe>,
    e: Option<Pipe>,
}

impl Default for PipeCell {
    fn default() -> Self {
        Self { n: None, s: None, w: None, e: None, }
    }
}

impl PipeCell {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn lay_down(&mut self, dir: Direction, pipe: Pipe) -> Option<Pipe> {
        match dir {
            Direction::North => self.n.replace(pipe),
            Direction::South => self.s.replace(pipe),
            Direction::West => self.w.replace(pipe),
            Direction::East => self.e.replace(pipe),
        }
    }

    pub fn dig_up(&mut self, dir: Direction) -> Option<Pipe> {
        match dir {
            Direction::North => self.n.take(),
            Direction::South => self.s.take(),
            Direction::West => self.w.take(),
            Direction::East => self.e.take(),
        }
    }

    pub fn char(&self) -> char {
        match (self.n, self.s, self.w, self.e) {
            (             None,              None,              None,              None,) => XN_XS_XW_XE,
            (             None,              None,              None, Some(Pipe::Empty),) => XN_XS_XW_EE,
            (             None,              None,              None,  Some(Pipe::Full),) => XN_XS_XW_FE,
            (             None,              None, Some(Pipe::Empty),              None,) => XN_XS_EW_XE,
            (             None,              None, Some(Pipe::Empty), Some(Pipe::Empty),) => XN_XS_EW_EE,
            (             None,              None, Some(Pipe::Empty),  Some(Pipe::Full),) => XN_XS_EW_FE,
            (             None,              None,  Some(Pipe::Full),              None,) => XN_XS_FW_XE,
            (             None,              None,  Some(Pipe::Full), Some(Pipe::Empty),) => XN_XS_FW_EE,
            (             None,              None,  Some(Pipe::Full),  Some(Pipe::Full),) => XN_XS_FW_FE,
            (             None, Some(Pipe::Empty),              None,              None,) => XN_ES_XW_XE,
            (             None, Some(Pipe::Empty),              None, Some(Pipe::Empty),) => XN_ES_XW_EE,
            (             None, Some(Pipe::Empty),              None,  Some(Pipe::Full),) => XN_ES_XW_FE,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty),              None,) => XN_ES_EW_XE,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),) => XN_ES_EW_EE,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),) => XN_ES_EW_FE,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full),              None,) => XN_ES_FW_XE,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),) => XN_ES_FW_EE,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),) => XN_ES_FW_FE,
            (             None,  Some(Pipe::Full),              None,              None,) => XN_FS_XW_XE,
            (             None,  Some(Pipe::Full),              None, Some(Pipe::Empty),) => XN_FS_XW_EE,
            (             None,  Some(Pipe::Full),              None,  Some(Pipe::Full),) => XN_FS_XW_FE,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty),              None,) => XN_FS_EW_XE,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),) => XN_FS_EW_EE,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),) => XN_FS_EW_FE,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full),              None,) => XN_FS_FW_XE,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),) => XN_FS_FW_EE,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),) => XN_FS_FW_FE,
            (Some(Pipe::Empty),              None,              None,              None,) => EN_XS_XW_XE,
            (Some(Pipe::Empty),              None,              None, Some(Pipe::Empty),) => EN_XS_XW_EE,
            (Some(Pipe::Empty),              None,              None,  Some(Pipe::Full),) => EN_XS_XW_FE,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty),              None,) => EN_XS_EW_XE,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty), Some(Pipe::Empty),) => EN_XS_EW_EE,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty),  Some(Pipe::Full),) => EN_XS_EW_FE,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full),              None,) => EN_XS_FW_XE,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full), Some(Pipe::Empty),) => EN_XS_FW_EE,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full),  Some(Pipe::Full),) => EN_XS_FW_FE,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None,              None,) => EN_ES_XW_XE,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None, Some(Pipe::Empty),) => EN_ES_XW_EE,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None,  Some(Pipe::Full),) => EN_ES_XW_FE,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),              None,) => EN_ES_EW_XE,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),) => EN_ES_EW_EE,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),) => EN_ES_EW_FE,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),              None,) => EN_ES_FW_XE,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),) => EN_ES_FW_EE,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),) => EN_ES_FW_FE,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None,              None,) => EN_FS_XW_XE,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None, Some(Pipe::Empty),) => EN_FS_XW_EE,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None,  Some(Pipe::Full),) => EN_FS_XW_FE,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),              None,) => EN_FS_EW_XE,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),) => EN_FS_EW_EE,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),) => EN_FS_EW_FE,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),              None,) => EN_FS_FW_XE,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),) => EN_FS_FW_EE,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),) => EN_FS_FW_FE,
            ( Some(Pipe::Full),              None,              None,              None,) => FN_XS_XW_XE,
            ( Some(Pipe::Full),              None,              None, Some(Pipe::Empty),) => FN_XS_XW_EE,
            ( Some(Pipe::Full),              None,              None,  Some(Pipe::Full),) => FN_XS_XW_FE,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty),              None,) => FN_XS_EW_XE,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty), Some(Pipe::Empty),) => FN_XS_EW_EE,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty),  Some(Pipe::Full),) => FN_XS_EW_FE,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full),              None,) => FN_XS_FW_XE,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full), Some(Pipe::Empty),) => FN_XS_FW_EE,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full),  Some(Pipe::Full),) => FN_XS_FW_FE,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None,              None,) => FN_ES_XW_XE,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None, Some(Pipe::Empty),) => FN_ES_XW_EE,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None,  Some(Pipe::Full),) => FN_ES_XW_FE,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),              None,) => FN_ES_EW_XE,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),) => FN_ES_EW_EE,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),) => FN_ES_EW_FE,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),              None,) => FN_ES_FW_XE,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),) => FN_ES_FW_EE,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),) => FN_ES_FW_FE,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None,              None,) => FN_FS_XW_XE,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None, Some(Pipe::Empty),) => FN_FS_XW_EE,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None,  Some(Pipe::Full),) => FN_FS_XW_FE,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),              None,) => FN_FS_EW_XE,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),) => FN_FS_EW_EE,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),) => FN_FS_EW_FE,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),              None,) => FN_FS_FW_XE,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),) => FN_FS_FW_EE,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),) => FN_FS_FW_FE,
        }
    }
}
