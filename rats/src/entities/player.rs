use super::{dir, Dimensions, Direction, Entity, Position, State};
use crate::{game_context::Action, maze::Maze};
use video::{
    SizeWrapping, ATTR_NONE, PLAYER_DOWN, PLAYER_LEFT, PLAYER_RIGHT, PLAYER_UP,
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

    pub fn can_advance(&self, maze: &Maze, direction: Direction) -> bool {
        let (row, col) = (self.pos.row, self.pos.col);
        let (rows, cols) = (maze.rows(), maze.cols());
        match direction {
            dir::DOWN => !maze.is_wall_quad(row.inc(rows), col),
            dir::DOWN_LEFT => !maze.is_wall_quad(row.inc(rows), col.dec(cols)),
            dir::DOWN_RIGHT => !maze.is_wall_quad(row.inc(rows), col.inc(cols)),
            dir::UP => !maze.is_wall_quad(row.dec(rows), col),
            dir::UP_LEFT => !maze.is_wall_quad(row.dec(rows), col.dec(cols)),
            dir::UP_RIGHT => !maze.is_wall_quad(row.dec(rows), col.inc(cols)),
            dir::LEFT => !maze.is_wall_quad(row, col.dec(cols)),
            dir::RIGHT => !maze.is_wall_quad(row, col.inc(cols)),
            _ => false,
        }
    }
}

pub fn render_player(player: &Player, maze: &mut Maze) {
    let offset = if player.dir == dir::NONE {
        0
    } else {
        (player.cycle >> 1) + 1
    };
    let ch = match player.dir {
        dir::DOWN => PLAYER_DOWN,
        dir::DOWN_LEFT => PLAYER_LEFT,
        dir::DOWN_RIGHT => PLAYER_RIGHT,
        dir::UP => PLAYER_UP,
        dir::UP_LEFT => PLAYER_LEFT,
        dir::UP_RIGHT => PLAYER_RIGHT,
        dir::LEFT => PLAYER_LEFT,
        dir::RIGHT => PLAYER_RIGHT,
        _ => PLAYER_DOWN,
    } + offset * 4;
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
    if player.can_advance(maze, player.dir) {
        player.advance(player.dir, maze.dimensions);
    } else {
        if player.dir & dir::UP != 0 && player.can_advance(maze, dir::UP) {
            player.advance(dir::UP, maze.dimensions);
        }
        if player.dir & dir::DOWN != 0 && player.can_advance(maze, dir::DOWN) {
            player.advance(dir::DOWN, maze.dimensions);
        }
        if player.dir & dir::LEFT != 0 && player.can_advance(maze, dir::LEFT) {
            player.advance(dir::LEFT, maze.dimensions);
        }
        if player.dir & dir::RIGHT != 0 && player.can_advance(maze, dir::RIGHT)
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
