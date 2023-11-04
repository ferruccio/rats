use video::{
    Size, SizeWrapping, ATTR_NONE, BIG_BLANK_START, BIG_BOOM_A1, BIG_BOOM_A2,
    RATS_DOWN_A1, RATS_DOWN_A2, RATS_LEFT_A1, RATS_LEFT_A2, RATS_RIGHT_A1,
    RATS_RIGHT_A2, RATS_UP_A1, RATS_UP_A2,
};

use super::{
    dir, state, Brat, Dimensions, Direction, Entity, EntityAction, Position,
    State,
};
use crate::{
    game_context::{flip_a_coin, random, random_direction, Action},
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

impl EntityAction for Rat {
    fn hit(&self, pos: Position, dims: Dimensions) -> bool {
        if pos == self.pos {
            return true;
        }
        let row_1 = self.pos.row.inc(dims.rows);
        let col_1 = self.pos.col.inc(dims.cols);
        pos == Position {
            row: pos.row,
            col: col_1,
        } || pos
            == Position {
                row: row_1,
                col: pos.col,
            }
            || pos
                == Position {
                    row: row_1,
                    col: col_1,
                }
    }

    fn explode(&mut self) {
        self.state = state::EXPLODING1;
    }
}

pub fn render_rat(rat: &Rat, maze: &mut Maze) {
    let ch = match rat.state {
        state::ALIVE => match (rat.dir, (rat.cycle) & 0x1 != 0) {
            (dir::UP, false) => RATS_UP_A1,
            (dir::UP, true) => RATS_UP_A2,
            (dir::DOWN, false) => RATS_DOWN_A1,
            (dir::DOWN, true) => RATS_DOWN_A2,
            (dir::LEFT, false) => RATS_LEFT_A1,
            (dir::LEFT, true) => RATS_LEFT_A2,
            (_, false) => RATS_RIGHT_A1,
            (_, true) => RATS_RIGHT_A2,
        },
        state::EXPLODING1 => BIG_BOOM_A1,
        state::EXPLODING2 => BIG_BOOM_A2,
        state::EXPLODING3 => BIG_BOOM_A1,
        state::DEAD | _ => BIG_BLANK_START,
    };
    maze.buffer
        .set_quad(rat.pos.row, rat.pos.col, ch, ATTR_NONE);
}

// frames per unit of brat animation
const RAT_FRAMES: u32 = 8;

pub fn update_rat(rat: &Rat, maze: &Maze, frames: u32, spawn: bool) -> Action {
    if frames < rat.updated + RAT_FRAMES {
        return Action::Nothing;
    }
    let mut rat = *rat;
    match rat.state {
        state::ALIVE => {
            if spawn && flip_a_coin() {
                return Action::New(Entity::Brat(Brat {
                    updated: frames,
                    distance: 10 + random(10, 20),
                    pos: rat.pos,
                    dir: random_direction(),
                    state: state::ALIVE,
                    cycle: 0,
                }));
            }
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
        state::EXPLODING1 => Action::Update(Entity::Rat(Rat {
            updated: frames + RAT_FRAMES / 2,
            state: state::EXPLODING2,
            ..rat
        })),
        state::EXPLODING2 => Action::Update(Entity::Rat(Rat {
            updated: frames + RAT_FRAMES / 2,
            state: state::EXPLODING3,
            ..rat
        })),
        state::EXPLODING3 => Action::Update(Entity::Rat(Rat {
            updated: frames + RAT_FRAMES / 2,
            state: state::DEAD,
            ..rat
        })),
        state::DEAD | _ => Action::Delete,
    }
}
