use video::{
    Size, ATTR_NONE, BRATS_DOWN_A1, BRATS_DOWN_A2, BRATS_LEFT_A1,
    BRATS_LEFT_A2, BRATS_RIGHT_A1, BRATS_RIGHT_A2, BRATS_UP_A1, BRATS_UP_A2,
};

use super::{dir, Dimensions, Direction, Entity, Position, State};
use crate::{
    game_context::{random, random_direction, Action},
    maze::Maze,
};

#[derive(Debug, Clone, Copy)]
pub struct Brat {
    pub updated: u32,
    pub distance: Size,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
    pub cycle: u8,
}

impl Brat {
    pub fn advance(&mut self, dir: Direction, dims: Dimensions) {
        self.pos = self.pos.advance(dir, dims);
    }

    pub fn can_advance(&self, maze: &Maze, dir: Direction) -> bool {
        let mut player = *self;
        player.advance(dir, maze.dimensions);
        let (row, col) = (player.pos.row, player.pos.col);
        !maze.is_wall(row, col)
    }
}

pub fn render_brat(brat: &Brat, maze: &mut Maze) {
    let ch = match (brat.dir, (brat.cycle) & 0x1 != 0) {
        (dir::UP, false) => BRATS_UP_A1,
        (dir::UP, true) => BRATS_UP_A2,
        (dir::DOWN, false) => BRATS_DOWN_A1,
        (dir::DOWN, true) => BRATS_DOWN_A2,
        (dir::LEFT, false) => BRATS_LEFT_A1,
        (dir::LEFT, true) => BRATS_LEFT_A2,
        (_, false) => BRATS_RIGHT_A1,
        (_, true) => BRATS_RIGHT_A2,
    };
    maze.buffer
        .set_chattr(brat.pos.row, brat.pos.col, ch, ATTR_NONE);
}

// frames per unit of brat animation
const BRAT_FRAMES: u32 = 10;

pub fn update_brat(brat: &Brat, maze: &Maze, frames: u32) -> Action {
    if frames < brat.updated + BRAT_FRAMES {
        return Action::Nothing;
    }
    let mut brat = *brat;
    if brat.distance == 0 || !brat.can_advance(maze, brat.dir) {
        brat.dir = random_direction();
        brat.distance = random(5, 15);
    } else {
        brat.advance(brat.dir, maze.dimensions);
        brat.distance -= 1;
    }
    Action::Update(Entity::Brat(Brat {
        updated: frames + BRAT_FRAMES,
        cycle: (brat.cycle + 1) & 0x3,
        ..brat
    }))
}
