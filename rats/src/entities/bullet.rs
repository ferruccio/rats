use super::{
    dir, Direction, Entity, EntityAction, Position, State, BULLET_UPDATE_MS,
};
use crate::{
    game_context::Action,
    maze::{with_pristine_maze, Maze},
};
use video::{
    SizeWrapping, ATTR_NONE, BULLET_DOWN, BULLET_DOWN_LEFT, BULLET_DOWN_RIGHT,
    BULLET_LEFT, BULLET_RIGHT, BULLET_UP, BULLET_UP_LEFT, BULLET_UP_RIGHT,
    LIL_BOOM_A1, LIL_BOOM_A2,
};

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub update: u32,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
}

impl EntityAction for Bullet {
    fn hit(&self, pos: Position) -> bool {
        self.state == State::Alive && self.pos == pos
    }

    fn explode(&mut self) {
        self.state = State::Exploding1;
    }
}

pub fn render_bullet(bullet: &Bullet, maze: &mut Maze) {
    let ch = match bullet.state {
        State::Alive => match bullet.dir {
            dir::DOWN => BULLET_DOWN,
            dir::DOWN_LEFT => BULLET_DOWN_LEFT,
            dir::DOWN_RIGHT => BULLET_DOWN_RIGHT,
            dir::UP => BULLET_UP,
            dir::UP_LEFT => BULLET_UP_LEFT,
            dir::UP_RIGHT => BULLET_UP_RIGHT,
            dir::LEFT => BULLET_LEFT,
            dir::RIGHT => BULLET_RIGHT,
            _ => b'?',
        },
        State::Exploding1 => LIL_BOOM_A1,
        State::Exploding2 => LIL_BOOM_A2,
        State::Exploding3 => LIL_BOOM_A1,
        State::Dead => b'*',
    };
    maze.buffer
        .set_chattr(bullet.pos.row, bullet.pos.col, ch, ATTR_NONE);
}

pub fn update_bullet(bullet: &Bullet, update: u32) -> Action {
    if update < bullet.update + BULLET_UPDATE_MS {
        return Action::Nothing;
    }
    with_pristine_maze(|maze| {
        let bullet = *bullet;
        match bullet.state {
            State::Alive => {
                let (row, col) = (bullet.pos.row, bullet.pos.col);
                let (rows, cols) = (maze.rows(), maze.cols());
                let (row, col) = match bullet.dir {
                    dir::DOWN => (row.inc(rows), col),
                    dir::DOWN_LEFT => (row.inc(rows), col.dec(cols)),
                    dir::DOWN_RIGHT => (row.inc(rows), col.inc(cols)),
                    dir::UP => (row.dec(rows), col),
                    dir::UP_LEFT => (row.dec(rows), col.dec(cols)),
                    dir::UP_RIGHT => (row.dec(rows), col.inc(cols)),
                    dir::LEFT => (row, col.dec(cols)),
                    dir::RIGHT => (row, col.inc(cols)),
                    _ => (row, col),
                };
                if maze.is_wall(row, col) {
                    Action::Delete
                } else {
                    Action::Update(Entity::Bullet(Bullet {
                        update: update + BULLET_UPDATE_MS,
                        pos: Position { row, col },
                        ..bullet
                    }))
                }
            }
            State::Exploding1 => Action::Update(Entity::Bullet(Bullet {
                update: update + BULLET_UPDATE_MS,
                state: State::Exploding2,
                ..bullet
            })),
            State::Exploding2 => Action::Update(Entity::Bullet(Bullet {
                update: update + BULLET_UPDATE_MS,
                state: State::Exploding3,
                ..bullet
            })),
            State::Exploding3 => Action::Update(Entity::Bullet(Bullet {
                update: update + BULLET_UPDATE_MS,
                state: State::Dead,
                ..bullet
            })),
            State::Dead => Action::Delete,
        }
    })
}
