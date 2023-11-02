use video::{
    Size, SizeWrapping, ATTR_NONE, RATS_DOWN_A1, RATS_DOWN_A2, RATS_LEFT_A1,
    RATS_RIGHT_A1, RATS_RIGHT_A2, RATS_UP_A1, RATS_UP_A2,
};

use super::{dir, Dimensions, Direction, Entity, Position, State};
use crate::{
    game_context::{random, random_direction, Action},
    maze::Maze,
};

#[derive(Debug, Clone, Copy)]
pub struct Rat {
    pub updated: u32,
    pub distance: Size,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
    pub cycle: u8,
}

impl Rat {
    pub fn advance(&mut self, dir: Direction, dims: Dimensions) {
        self.pos = self.pos.advance(dir, dims);
    }

    pub fn can_advance(&self, maze: &Maze, dir: Direction) -> bool {
        let mut player = *self;
        player.advance(dir, maze.dimensions);
        let (row1, col1) = (player.pos.row, player.pos.col);
        let row2 = player.pos.row.inc(maze.dimensions.rows);
        let col2 = player.pos.col.inc(maze.dimensions.cols);
        !(((dir & dir::UP) != 0
            && (maze.is_wall(row1, col1) || maze.is_wall(row1, col2)))
            || ((dir & dir::DOWN) != 0
                && (maze.is_wall(row2, col1) || maze.is_wall(row2, col2)))
            || ((dir & dir::LEFT) != 0
                && (maze.is_wall(row1, col1) || maze.is_wall(row2, col1)))
            || ((dir & dir::RIGHT) != 0
                && (maze.is_wall(row1, col2) || maze.is_wall(row2, col2))))
    }
}

pub fn render_rat(rat: &Rat, maze: &mut Maze) {
    let ch = match (rat.dir, (rat.cycle) & 0x1 != 0) {
        (dir::UP, false) => RATS_UP_A1,
        (dir::UP, true) => RATS_UP_A2,
        (dir::DOWN, false) => RATS_DOWN_A1,
        (dir::DOWN, true) => RATS_DOWN_A2,
        (dir::LEFT, false) => RATS_LEFT_A1,
        (dir::LEFT, true) => RATS_LEFT_A1,
        (_, false) => RATS_RIGHT_A1,
        (_, true) => RATS_RIGHT_A2,
    };
    maze.buffer
        .set_quad(rat.pos.row, rat.pos.col, ch, ATTR_NONE);
}

// frames per unit of brat animation
const RAT_FRAMES: u32 = 15;

pub fn update_rat(rat: &Rat, maze: &Maze, frames: u32) -> Action {
    if frames < rat.updated + RAT_FRAMES {
        return Action::Nothing;
    }
    let mut rat = *rat;
    if rat.distance == 0 || !rat.can_advance(maze, rat.dir) {
        rat.dir = random_direction();
        rat.distance = random(5, 15);
    } else {
        rat.advance(rat.dir, maze.dimensions);
        rat.distance -= 1;
    }
    Action::Update(Entity::Rat(Rat {
        updated: frames + RAT_FRAMES,
        cycle: (rat.cycle + 1) & 0x3,
        ..rat
    }))
}
