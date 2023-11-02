use super::{Direction, Position, State};

#[derive(Debug, Clone, Copy)]
pub struct Rat {
    pub updated: u32,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
    pub cycle: u8,
}
