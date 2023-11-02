use super::{Position, State};

#[derive(Debug, Clone, Copy)]
pub struct RatFactory {
    pub updated: u32,
    pub pos: Position,
    pub state: State,
    pub cycle: u8,
}
