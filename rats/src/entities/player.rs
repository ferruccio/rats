use super::{
    dir, state, Dimensions, Direction, Entity, EntityAction, Position, State,
};
use crate::{game_context::Action, maze::Maze};
use video::{
    SizeWrapping, ATTR_NONE, BIG_BLANK_START, BIG_BOOM_A1, BIG_BOOM_A2,
    PLAYER_DOWN, PLAYER_LEFT, PLAYER_RIGHT, PLAYER_UP,
};

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub updated: u32,
    pub pos: Position,
    pub dir: Direction,
    pub stop_dir: Direction,
    pub state: State,
    pub cycle: u8,
}

impl Player {
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

    pub fn effective_dir(&self) -> Direction {
        let mut dir = self.dir;
        if (dir & dir::UP) != 0 && (dir & dir::DOWN) != 0 {
            dir &= !(dir::UP | dir::DOWN);
        }
        if (dir & dir::LEFT) != 0 && (dir & dir::RIGHT) != 0 {
            dir &= !(dir::LEFT | dir::RIGHT);
        }
        dir
    }
}

impl EntityAction for Player {
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

pub fn render_player(player: &Player, maze: &mut Maze) {
    let ch = match player.state {
        state::ALIVE => {
            let dir = match player.effective_dir() {
                dir::NONE => player.stop_dir,
                dir => dir,
            };
            let offset = if player.effective_dir() == dir::NONE {
                0
            } else {
                (player.cycle >> 1) + 1
            };
            let ch = match dir {
                dir::DOWN_LEFT => PLAYER_LEFT,
                dir::DOWN_RIGHT => PLAYER_RIGHT,
                dir::UP => PLAYER_UP,
                dir::UP_LEFT => PLAYER_LEFT,
                dir::UP_RIGHT => PLAYER_RIGHT,
                dir::LEFT => PLAYER_LEFT,
                dir::RIGHT => PLAYER_RIGHT,
                dir::DOWN | _ => PLAYER_DOWN,
            } + offset * 4;
            ch
        }
        state::EXPLODING1 => BIG_BOOM_A1,
        state::EXPLODING2 => BIG_BOOM_A2,
        state::EXPLODING3 => BIG_BOOM_A1,
        state::DEAD | _ => BIG_BLANK_START,
    };
    maze.buffer
        .set_quad(player.pos.row, player.pos.col, ch, ATTR_NONE);
}

// frames per unit of player motion
const PLAYER_FRAMES: u32 = 5;

pub fn update_player(player: &Player, maze: &Maze, frames: u32) -> Action {
    if frames < player.updated + PLAYER_FRAMES {
        return Action::Nothing;
    }
    let mut player = *player;
    match player.state {
        state::ALIVE => {
            if player.can_advance(maze, player.dir) {
                player.advance(player.dir, maze.dimensions);
            } else {
                if player.dir & dir::UP != 0
                    && player.can_advance(maze, dir::UP)
                {
                    player.advance(dir::UP, maze.dimensions);
                }
                if player.dir & dir::DOWN != 0
                    && player.can_advance(maze, dir::DOWN)
                {
                    player.advance(dir::DOWN, maze.dimensions);
                }
                if player.dir & dir::LEFT != 0
                    && player.can_advance(maze, dir::LEFT)
                {
                    player.advance(dir::LEFT, maze.dimensions);
                }
                if player.dir & dir::RIGHT != 0
                    && player.can_advance(maze, dir::RIGHT)
                {
                    player.advance(dir::RIGHT, maze.dimensions);
                }
            }
            Action::Update(Entity::Player(Player {
                updated: frames + PLAYER_FRAMES,
                cycle: (player.cycle + 1) & 0x3,
                ..player
            }))
        }
        state::EXPLODING1 => Action::Update(Entity::Player(Player {
            updated: frames + PLAYER_FRAMES / 2,
            state: state::EXPLODING2,
            ..player
        })),
        state::EXPLODING2 => Action::Update(Entity::Player(Player {
            updated: frames + PLAYER_FRAMES / 2,
            state: state::EXPLODING3,
            ..player
        })),
        state::EXPLODING3 => Action::Update(Entity::Player(Player {
            updated: frames + PLAYER_FRAMES / 2,
            state: state::DEAD,
            ..player
        })),
        state::DEAD | _ => Action::Update(Entity::Player(Player {
            updated: frames + PLAYER_FRAMES * 2,
            state: state::ALIVE,
            ..player
        })),
    }
}
