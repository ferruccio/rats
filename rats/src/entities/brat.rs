use super::{Direction, Position, State};
use crate::{game_context::Action, maze::Maze};

#[derive(Debug, Clone, Copy)]
pub struct Brat {
    pub updated: u32,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
    pub cycle: u8,
}

pub fn render_brat(rat: &Brat, maze: &mut Maze) {}

// frames per unit of brat animation
const BRAT_FRAMES: u32 = 10;

pub fn update_brat(brat: &Brat, maze: &Maze, frames: u32) -> Action {
    if frames < brat.updated + BRAT_FRAMES {
        return Action::Nothing;
    }
    Action::Nothing
}
