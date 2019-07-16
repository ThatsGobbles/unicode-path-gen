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

impl PipeCell {
    pub fn char(&self) -> char {
        match (self.u, self.d, self.l, self.r) {
            (             None,              None,              None,              None,)
                => XU_XD_XL_XR,
            (             None,              None,              None, Some(Pipe::Empty),)
                => XU_XD_XL_ER,
            (             None,              None,              None,  Some(Pipe::Full),)
                => XU_XD_XL_FR,
            (             None,              None, Some(Pipe::Empty),              None,)
                => XU_XD_EL_XR,
            (             None,              None, Some(Pipe::Empty), Some(Pipe::Empty),)
                => XU_XD_EL_ER,
            (             None,              None, Some(Pipe::Empty),  Some(Pipe::Full),)
                => XU_XD_EL_FR,
            (             None,              None,  Some(Pipe::Full),              None,)
                => XU_XD_FL_XR,
            (             None,              None,  Some(Pipe::Full), Some(Pipe::Empty),)
                => XU_XD_FL_ER,
            (             None,              None,  Some(Pipe::Full),  Some(Pipe::Full),)
                => XU_XD_FL_FR,
            (             None, Some(Pipe::Empty),              None,              None,)
                => XU_ED_XL_XR,
            (             None, Some(Pipe::Empty),              None, Some(Pipe::Empty),)
                => XU_ED_XL_ER,
            (             None, Some(Pipe::Empty),              None,  Some(Pipe::Full),)
                => XU_ED_XL_FR,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty),              None,)
                => XU_ED_EL_XR,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),)
                => XU_ED_EL_ER,
            (             None, Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),)
                => XU_ED_EL_FR,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full),              None,)
                => XU_ED_FL_XR,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),)
                => XU_ED_FL_ER,
            (             None, Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),)
                => XU_ED_FL_FR,
            (             None,  Some(Pipe::Full),              None,              None,)
                => XU_FD_XL_XR,
            (             None,  Some(Pipe::Full),              None, Some(Pipe::Empty),)
                => XU_FD_XL_ER,
            (             None,  Some(Pipe::Full),              None,  Some(Pipe::Full),)
                => XU_FD_XL_FR,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty),              None,)
                => XU_FD_EL_XR,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),)
                => XU_FD_EL_ER,
            (             None,  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),)
                => XU_FD_EL_FR,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full),              None,)
                => XU_FD_FL_XR,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),)
                => XU_FD_FL_ER,
            (             None,  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),)
                => XU_FD_FL_FR,
            (Some(Pipe::Empty),              None,              None,              None,)
                => EU_XD_XL_XR,
            (Some(Pipe::Empty),              None,              None, Some(Pipe::Empty),)
                => EU_XD_XL_ER,
            (Some(Pipe::Empty),              None,              None,  Some(Pipe::Full),)
                => EU_XD_XL_FR,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty),              None,)
                => EU_XD_EL_XR,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty), Some(Pipe::Empty),)
                => EU_XD_EL_ER,
            (Some(Pipe::Empty),              None, Some(Pipe::Empty),  Some(Pipe::Full),)
                => EU_XD_EL_FR,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full),              None,)
                => EU_XD_FL_XR,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full), Some(Pipe::Empty),)
                => EU_XD_FL_ER,
            (Some(Pipe::Empty),              None,  Some(Pipe::Full),  Some(Pipe::Full),)
                => EU_XD_FL_FR,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None,              None,)
                => EU_ED_XL_XR,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None, Some(Pipe::Empty),)
                => EU_ED_XL_ER,
            (Some(Pipe::Empty), Some(Pipe::Empty),              None,  Some(Pipe::Full),)
                => EU_ED_XL_FR,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),              None,)
                => EU_ED_EL_XR,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),)
                => EU_ED_EL_ER,
            (Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),)
                => EU_ED_EL_FR,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),              None,)
                => EU_ED_FL_XR,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),)
                => EU_ED_FL_ER,
            (Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),)
                => EU_ED_FL_FR,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None,              None,)
                => EU_FD_XL_XR,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None, Some(Pipe::Empty),)
                => EU_FD_XL_ER,
            (Some(Pipe::Empty),  Some(Pipe::Full),              None,  Some(Pipe::Full),)
                => EU_FD_XL_FR,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),              None,)
                => EU_FD_EL_XR,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),)
                => EU_FD_EL_ER,
            (Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),)
                => EU_FD_EL_FR,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),              None,)
                => EU_FD_FL_XR,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),)
                => EU_FD_FL_ER,
            (Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),)
                => EU_FD_FL_FR,
            ( Some(Pipe::Full),              None,              None,              None,)
                => FU_XD_XL_XR,
            ( Some(Pipe::Full),              None,              None, Some(Pipe::Empty),)
                => FU_XD_XL_ER,
            ( Some(Pipe::Full),              None,              None,  Some(Pipe::Full),)
                => FU_XD_XL_FR,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty),              None,)
                => FU_XD_EL_XR,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty), Some(Pipe::Empty),)
                => FU_XD_EL_ER,
            ( Some(Pipe::Full),              None, Some(Pipe::Empty),  Some(Pipe::Full),)
                => FU_XD_EL_FR,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full),              None,)
                => FU_XD_FL_XR,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full), Some(Pipe::Empty),)
                => FU_XD_FL_ER,
            ( Some(Pipe::Full),              None,  Some(Pipe::Full),  Some(Pipe::Full),)
                => FU_XD_FL_FR,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None,              None,)
                => FU_ED_XL_XR,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None, Some(Pipe::Empty),)
                => FU_ED_XL_ER,
            ( Some(Pipe::Full), Some(Pipe::Empty),              None,  Some(Pipe::Full),)
                => FU_ED_XL_FR,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),              None,)
                => FU_ED_EL_XR,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty), Some(Pipe::Empty),)
                => FU_ED_EL_ER,
            ( Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),  Some(Pipe::Full),)
                => FU_ED_EL_FR,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),              None,)
                => FU_ED_FL_XR,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full), Some(Pipe::Empty),)
                => FU_ED_FL_ER,
            ( Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),  Some(Pipe::Full),)
                => FU_ED_FL_FR,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None,              None,)
                => FU_FD_XL_XR,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None, Some(Pipe::Empty),)
                => FU_FD_XL_ER,
            ( Some(Pipe::Full),  Some(Pipe::Full),              None,  Some(Pipe::Full),)
                => FU_FD_XL_FR,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),              None,)
                => FU_FD_EL_XR,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty), Some(Pipe::Empty),)
                => FU_FD_EL_ER,
            ( Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),  Some(Pipe::Full),)
                => FU_FD_EL_FR,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),              None,)
                => FU_FD_FL_XR,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full), Some(Pipe::Empty),)
                => FU_FD_FL_ER,
            ( Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),  Some(Pipe::Full),)
                => FU_FD_FL_FR,
        }
    }
}
