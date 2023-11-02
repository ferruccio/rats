use super::{dir, Direction, Entity, Position, State};
use crate::{game_context::Action, maze::Maze};
use video::{
    SizeWrapping, ATTR_NONE, BULLET_DOWN, BULLET_DOWN_LEFT, BULLET_DOWN_RIGHT,
    BULLET_LEFT, BULLET_RIGHT, BULLET_UP, BULLET_UP_LEFT, BULLET_UP_RIGHT,
};

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub updated: u32,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
}

pub fn render_bullet(bullet: &Bullet, maze: &mut Maze) {
    let var_name = match bullet.dir {
        dir::DOWN => BULLET_DOWN,
        dir::DOWN_LEFT => BULLET_DOWN_LEFT,
        dir::DOWN_RIGHT => BULLET_DOWN_RIGHT,
        dir::UP => BULLET_UP,
        dir::UP_LEFT => BULLET_UP_LEFT,
        dir::UP_RIGHT => BULLET_UP_RIGHT,
        dir::LEFT => BULLET_LEFT,
        dir::RIGHT => BULLET_RIGHT,
        _ => b'?',
    };
    let ch = var_name;
    maze.buffer
        .set_chattr(bullet.pos.row, bullet.pos.col, ch, ATTR_NONE);
}

// frames per unit of bullet motion
const BULLET_FRAMES: u32 = 3;

pub fn update_bullet(bullet: &Bullet, maze: &Maze, frames: u32) -> Action {
    if frames < bullet.updated + BULLET_FRAMES {
        return Action::Nothing;
    }
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
            updated: frames + BULLET_FRAMES,
            pos: Position { row, col },
            ..*bullet
        }))
    }
}
