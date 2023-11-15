use video::{
    Size, SizeWrapping, ATTR_NONE, BIG_BLANK_START, BIG_BOOM_A1, BIG_BOOM_A2,
    RATS_DOWN_A1, RATS_DOWN_A2, RATS_LEFT_A1, RATS_LEFT_A2, RATS_RIGHT_A1,
    RATS_RIGHT_A2, RATS_UP_A1, RATS_UP_A2,
};

use super::{
    dir, hit_player_1, Brat, Dimensions, Direction, Entity, EntityAction,
    Player, Position, State, RAT_UPDATE_MS,
};
use crate::{
    game_context::{
        distance_squared, flip_a_coin, random, random_direction, Action,
    },
    maze::Maze,
};

#[derive(Debug, Clone, Copy)]
pub struct Rat {
    pub update: u32,
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
        if self.state != State::Alive {
            return false;
        }
        if pos == self.pos {
            return true;
        }
        let row_1 = self.pos.row.inc(dims.rows);
        let col_1 = self.pos.col.inc(dims.cols);
        pos == Position {
            row: self.pos.row,
            col: col_1,
        } || pos
            == Position {
                row: row_1,
                col: self.pos.col,
            }
            || pos
                == Position {
                    row: row_1,
                    col: col_1,
                }
    }

    fn explode(&mut self) {
        self.state = State::Exploding1;
    }
}

pub fn render_rat(rat: &Rat, maze: &mut Maze) {
    let ch = match rat.state {
        State::Alive => match (rat.dir, (rat.cycle) & 0x1 != 0) {
            (dir::UP, false) => RATS_UP_A1,
            (dir::UP, true) => RATS_UP_A2,
            (dir::DOWN, false) => RATS_DOWN_A1,
            (dir::DOWN, true) => RATS_DOWN_A2,
            (dir::LEFT, false) => RATS_LEFT_A1,
            (dir::LEFT, true) => RATS_LEFT_A2,
            (_, false) => RATS_RIGHT_A1,
            (_, true) => RATS_RIGHT_A2,
        },
        State::Exploding1 => BIG_BOOM_A1,
        State::Exploding2 => BIG_BOOM_A2,
        State::Exploding3 => BIG_BOOM_A1,
        State::Dead => BIG_BLANK_START,
    };
    maze.buffer
        .set_quad(rat.pos.row, rat.pos.col, ch, ATTR_NONE);
}

pub fn update_rat(
    rat: &Rat,
    maze: &Maze,
    player: &Player,
    damage: usize,
    update: u32,
    spawn: bool,
) -> Action {
    if update < rat.update + RAT_UPDATE_MS {
        return Action::Nothing;
    }
    let mut rat = *rat;
    match rat.state {
        State::Alive => {
            if hit_player(rat.pos, maze.dimensions, player) {
                return Action::Attack(damage);
            }
            if spawn && flip_a_coin() {
                return Action::New(Entity::Brat(Brat {
                    update,
                    distance: 10 + random(10, 20),
                    pos: rat.pos,
                    dir: random_direction(),
                    state: State::Alive,
                    cycle: 0,
                }));
            }
            if let Some(dir) = player_dir(rat.pos, player.pos, maze.dimensions)
            {
                rat.dir = dir;
            }
            if rat.distance == 0 || !rat.can_advance(maze, rat.dir) {
                rat.dir = random_direction();
                rat.distance = random(5, 15);
            } else {
                rat.advance(rat.dir, maze.dimensions);
                rat.distance -= 1;
            }
            Action::Update(Entity::Rat(Rat {
                update: update + RAT_UPDATE_MS,
                cycle: (rat.cycle + 1) & 0x3,
                ..rat
            }))
        }
        State::Exploding1 => Action::Update(Entity::Rat(Rat {
            update: update + RAT_UPDATE_MS / 2,
            state: State::Exploding2,
            ..rat
        })),
        State::Exploding2 => Action::Update(Entity::Rat(Rat {
            update: update + RAT_UPDATE_MS / 2,
            state: State::Exploding3,
            ..rat
        })),
        State::Exploding3 => Action::Update(Entity::Rat(Rat {
            update: update + RAT_UPDATE_MS / 2,
            state: State::Dead,
            ..rat
        })),
        State::Dead => Action::Delete,
    }
}

fn hit_player(pos: Position, dims: Dimensions, player: &Player) -> bool {
    if hit_player_1(pos, dims, player) {
        return true;
    }
    let mut pos2 = pos;
    pos2.move_right(1, dims);
    if hit_player_1(pos2, dims, player) {
        return true;
    }
    let mut pos2 = pos;
    pos2.move_down(1, dims);
    if hit_player_1(pos2, dims, player) {
        return true;
    }
    pos2.move_right(1, dims);
    hit_player_1(pos2, dims, player)
}

pub fn player_dir(
    pos: Position,
    player_pos: Position,
    dims: Dimensions,
) -> Option<Direction> {
    if distance_squared(pos, player_pos, dims) < 100 {
        Some(pos.direction_to(player_pos, dims))
    } else {
        None
    }
}
