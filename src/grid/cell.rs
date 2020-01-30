use std::ops::BitOr;

use crate::grid::Direction;

const N0_E0_S0_W0: char = ' ';
const N0_E0_S0_W1: char = '╴';
const N0_E0_S1_W0: char = '╷';
const N0_E0_S1_W1: char = '┐';
const N0_E1_S0_W0: char = '╶';
const N0_E1_S0_W1: char = '─';
const N0_E1_S1_W0: char = '┌';
const N0_E1_S1_W1: char = '┬';
const N1_E0_S0_W0: char = '╵';
const N1_E0_S0_W1: char = '┘';
const N1_E0_S1_W0: char = '│';
const N1_E0_S1_W1: char = '┤';
const N1_E1_S0_W0: char = '└';
const N1_E1_S0_W1: char = '┴';
const N1_E1_S1_W0: char = '├';
const N1_E1_S1_W1: char = '┼';

// const XN_XE_XS_XW: char = ' ';
// const XN_EE_XS_XW: char = '╶';
// const XN_FE_XS_XW: char = '╺';
// const XN_XE_XS_EW: char = '╴';
// const XN_EE_XS_EW: char = '─';
// const XN_FE_XS_EW: char = '╼';
// const XN_XE_XS_FW: char = '╸';
// const XN_EE_XS_FW: char = '╾';
// const XN_FE_XS_FW: char = '━';
// const XN_XE_ES_XW: char = '╷';
// const XN_EE_ES_XW: char = '┌';
// const XN_FE_ES_XW: char = '┍';
// const XN_XE_ES_EW: char = '┐';
// const XN_EE_ES_EW: char = '┬';
// const XN_FE_ES_EW: char = '┮';
// const XN_XE_ES_FW: char = '┑';
// const XN_EE_ES_FW: char = '┭';
// const XN_FE_ES_FW: char = '┯';
// const XN_XE_FS_XW: char = '╻';
// const XN_EE_FS_XW: char = '┎';
// const XN_FE_FS_XW: char = '┏';
// const XN_XE_FS_EW: char = '┒';
// const XN_EE_FS_EW: char = '┰';
// const XN_FE_FS_EW: char = '┲';
// const XN_XE_FS_FW: char = '┓';
// const XN_EE_FS_FW: char = '┱';
// const XN_FE_FS_FW: char = '┳';
// const EN_XE_XS_XW: char = '╵';
// const EN_EE_XS_XW: char = '└';
// const EN_FE_XS_XW: char = '┕';
// const EN_XE_XS_EW: char = '┘';
// const EN_EE_XS_EW: char = '┴';
// const EN_FE_XS_EW: char = '┶';
// const EN_XE_XS_FW: char = '┙';
// const EN_EE_XS_FW: char = '┵';
// const EN_FE_XS_FW: char = '┷';
// const EN_XE_ES_XW: char = '│';
// const EN_EE_ES_XW: char = '├';
// const EN_FE_ES_XW: char = '┝';
// const EN_XE_ES_EW: char = '┤';
// const EN_EE_ES_EW: char = '┼';
// const EN_FE_ES_EW: char = '┾';
// const EN_XE_ES_FW: char = '┥';
// const EN_EE_ES_FW: char = '┽';
// const EN_FE_ES_FW: char = '┿';
// const EN_XE_FS_XW: char = '╽';
// const EN_EE_FS_XW: char = '┟';
// const EN_FE_FS_XW: char = '┢';
// const EN_XE_FS_EW: char = '┧';
// const EN_EE_FS_EW: char = '╁';
// const EN_FE_FS_EW: char = '╆';
// const EN_XE_FS_FW: char = '┪';
// const EN_EE_FS_FW: char = '╅';
// const EN_FE_FS_FW: char = '╈';
// const FN_XE_XS_XW: char = '╹';
// const FN_EE_XS_XW: char = '┖';
// const FN_FE_XS_XW: char = '┗';
// const FN_XE_XS_EW: char = '┚';
// const FN_EE_XS_EW: char = '┸';
// const FN_FE_XS_EW: char = '┺';
// const FN_XE_XS_FW: char = '┛';
// const FN_EE_XS_FW: char = '┹';
// const FN_FE_XS_FW: char = '┻';
// const FN_XE_ES_XW: char = '╿';
// const FN_EE_ES_XW: char = '┞';
// const FN_FE_ES_XW: char = '┡';
// const FN_XE_ES_EW: char = '┦';
// const FN_EE_ES_EW: char = '╀';
// const FN_FE_ES_EW: char = '╄';
// const FN_XE_ES_FW: char = '┩';
// const FN_EE_ES_FW: char = '╃';
// const FN_FE_ES_FW: char = '╇';
// const FN_XE_FS_XW: char = '┃';
// const FN_EE_FS_XW: char = '┠';
// const FN_FE_FS_XW: char = '┣';
// const FN_XE_FS_EW: char = '┨';
// const FN_EE_FS_EW: char = '╂';
// const FN_FE_FS_EW: char = '╊';
// const FN_XE_FS_FW: char = '┫';
// const FN_EE_FS_FW: char = '╉';
// const FN_FE_FS_FW: char = '╋';

// #[derive(Copy, Clone)]
// pub enum Pipe {
//     None,
//     Empty,
//     Full,
// }

// impl BitOr for Pipe {
//     type Output = Self;

//     fn bitor(self, rhs: Self) -> Self {
//         match (self, rhs) {
//             (Self::None, rhs) => rhs,
//             (lhs, Self::None) => lhs,
//             (Self::Empty, rhs) => rhs,
//             (lhs, Self::Empty) => lhs,
//             (Self::Full, Self::Full) => Self::Full,
//         }
//     }
// }

const CELL_N: Cell = Cell { n: true, e: false, s: false, w: false, };
const CELL_E: Cell = Cell { n: false, e: true, s: false, w: false, };
const CELL_S: Cell = Cell { n: false, e: false, s: true, w: false, };
const CELL_W: Cell = Cell { n: false, e: false, s: false, w: true, };

#[derive(Copy, Clone, Default)]
pub struct Cell {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
}

impl BitOr for Cell {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            n: self.n | rhs.n,
            e: self.e | rhs.e,
            s: self.s | rhs.s,
            w: self.w | rhs.w,
        }
    }
}

impl From<Direction> for Cell {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => CELL_N,
            Direction::East => CELL_E,
            Direction::South => CELL_S,
            Direction::West => CELL_W,
        }
    }
}

impl Cell {
    pub fn new(n: bool, e: bool, s: bool, w: bool) -> Self {
        Self { n, e, s, w, }
    }

    pub fn char(&self) -> char {
        match (self.n, self.e, self.s, self.w) {
            (false, false, false, false) => N0_E0_S0_W0,
            (false, false, false,  true) => N0_E0_S0_W1,
            (false, false,  true, false) => N0_E0_S1_W0,
            (false, false,  true,  true) => N0_E0_S1_W1,
            (false,  true, false, false) => N0_E1_S0_W0,
            (false,  true, false,  true) => N0_E1_S0_W1,
            (false,  true,  true, false) => N0_E1_S1_W0,
            (false,  true,  true,  true) => N0_E1_S1_W1,
            ( true, false, false, false) => N1_E0_S0_W0,
            ( true, false, false,  true) => N1_E0_S0_W1,
            ( true, false,  true, false) => N1_E0_S1_W0,
            ( true, false,  true,  true) => N1_E0_S1_W1,
            ( true,  true, false, false) => N1_E1_S0_W0,
            ( true,  true, false,  true) => N1_E1_S0_W1,
            ( true,  true,  true, false) => N1_E1_S1_W0,
            ( true,  true,  true,  true) => N1_E1_S1_W1,
        }
    }
}
