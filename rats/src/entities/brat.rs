use super::{
    dir, rat::player_dir, Direction, Entity, EntityAction, Player, Position,
    State,
};
use crate::{
    config::BRAT_UPDATE_MS,
    game_context::{random, random_direction, Action},
    maze::{with_pristine_maze, Maze},
};
use video::{
    Size, SizeWrapping, ATTR_NONE, BRATS_DOWN_A1, BRATS_DOWN_A2, BRATS_LEFT_A1,
    BRATS_LEFT_A2, BRATS_RIGHT_A1, BRATS_RIGHT_A2, BRATS_UP_A1, BRATS_UP_A2,
    LIL_BOOM_A1, LIL_BOOM_A2,
};

#[derive(Debug, Clone, Copy)]
pub struct Brat {
    pub update: u32,
    pub distance: Size,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
    pub cycle: u8,
}

impl Brat {
    pub fn advance(&mut self, dir: Direction) {
        self.pos = self.pos.advance(dir);
    }

    pub fn can_advance(&self, dir: Direction) -> bool {
        with_pristine_maze(|maze| {
            let mut player = *self;
            player.advance(dir);
            let (row, col) = (player.pos.row, player.pos.col);
            !maze.is_wall(row, col)
        })
    }
}

impl EntityAction for Brat {
    fn hit(&self, pos: Position) -> bool {
        self.state == State::Alive && self.pos == pos
    }

    fn explode(&mut self) {
        self.state = State::Exploding1;
    }
}

pub fn render_brat(brat: &Brat, maze: &mut Maze) {
    let ch = match brat.state {
        State::Alive => match (brat.dir, (brat.cycle) & 0x1 != 0) {
            (dir::UP, false) => BRATS_UP_A1,
            (dir::UP, true) => BRATS_UP_A2,
            (dir::DOWN, false) => BRATS_DOWN_A1,
            (dir::DOWN, true) => BRATS_DOWN_A2,
            (dir::LEFT, false) => BRATS_LEFT_A1,
            (dir::LEFT, true) => BRATS_LEFT_A2,
            (_, false) => BRATS_RIGHT_A1,
            (_, true) => BRATS_RIGHT_A2,
        },
        State::Exploding1 => LIL_BOOM_A1,
        State::Exploding2 => LIL_BOOM_A2,
        State::Exploding3 => LIL_BOOM_A1,
        State::Dead => b' ',
    };
    maze.buffer
        .set_chattr(brat.pos.row, brat.pos.col, ch, ATTR_NONE);
}

pub fn update_brat(
    brat: &Brat,
    player: &Player,
    damage: usize,
    update: u32,
) -> Action {
    if update < brat.update + BRAT_UPDATE_MS {
        return Action::Nothing;
    }
    let mut brat = *brat;
    match brat.state {
        State::Alive => {
            if hit_player_1(brat.pos, player) {
                return Action::Attack(damage);
            }
            if let Some(dir) = player_dir(brat.pos, player.pos) {
                brat.dir = dir;
            }
            if brat.distance == 0 || !brat.can_advance(brat.dir) {
                brat.dir = random_direction();
                brat.distance = random(5, 15);
            } else {
                brat.advance(brat.dir);
                brat.distance -= 1;
            }
            Action::Update(Entity::Brat(Brat {
                update: update + BRAT_UPDATE_MS,
                cycle: (brat.cycle + 1) & 0x3,
                ..brat
            }))
        }
        State::Exploding1 => Action::Update(Entity::Brat(Brat {
            update: update + BRAT_UPDATE_MS / 2,
            state: State::Exploding2,
            ..brat
        })),
        State::Exploding2 => Action::Update(Entity::Brat(Brat {
            update: update + BRAT_UPDATE_MS / 2,
            state: State::Exploding3,
            ..brat
        })),
        State::Exploding3 => Action::Update(Entity::Brat(Brat {
            update: update + BRAT_UPDATE_MS / 2,
            state: State::Dead,
            ..brat
        })),
        State::Dead => Action::Delete,
    }
}

pub fn hit_player_1(pos: Position, player: &Player) -> bool {
    if pos == player.pos {
        return true;
    }
    let (rows, cols) = with_pristine_maze(|maze| (maze.rows(), maze.cols()));
    let row_1 = player.pos.row.inc(rows);
    let col_1 = player.pos.col.inc(cols);
    pos == Position {
        row: player.pos.row,
        col: col_1,
    } || pos
        == Position {
            row: row_1,
            col: player.pos.col,
        }
        || pos
            == Position {
                row: row_1,
                col: col_1,
            }
}
