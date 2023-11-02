use crate::{
    entities::{
        Bullet, Dimensions, Direction, Entity, Index, Player, Position,
        DIR_DOWN, DIR_DOWN_LEFT, DIR_DOWN_RIGHT, DIR_LEFT, DIR_RIGHT, DIR_UP,
        DIR_UP_LEFT, DIR_UP_RIGHT,
    },
    game_context::GameContext,
    maze::Maze,
};
use video::SizeWrapping;

// frames per unit of motion
const PLAYER_FRAMES: u32 = 5;
const BULLET_FRAMES: u32 = 3;

enum Action {
    Nothing,
    Delete,
    Update(Entity),
}

impl GameContext {
    pub fn update(&mut self) {
        let mut actions: Vec<(Index, Action)> = vec![];
        for (index, entity) in self.entities.iter().enumerate() {
            let action = match entity {
                Entity::Player(player) => {
                    update_player(&player, &self.pristine_maze, self.frames)
                }
                Entity::Rat(_) => Action::Nothing,
                Entity::BabyRat(_) => Action::Nothing,
                Entity::RatFactory(_) => Action::Nothing,
                Entity::Bullet(bullet) => {
                    update_bullet(&bullet, &self.pristine_maze, self.frames)
                }
            };
            actions.push((index, action));
        }
        for (index, action) in actions.into_iter().rev() {
            match action {
                Action::Nothing => {}
                Action::Delete => {
                    let last = self.entities.len() - 1;
                    self.entities.swap(index, last);
                    self.entities.truncate(last);
                }
                Action::Update(entity) => {
                    self.entities[index] = entity;
                }
            };
        }
    }
}

impl Player {
    pub fn advance(&mut self, dir: Direction, dims: Dimensions) {
        self.pos = self.pos.advance(dir, dims);
    }

    pub fn can_advance(&self, maze: &Maze, direction: Direction) -> bool {
        let (row, col) = (self.pos.row, self.pos.col);
        let (rows, cols) = (maze.rows(), maze.cols());
        match direction {
            DIR_DOWN => !maze.is_wall_quad(row.inc(rows), col),
            DIR_DOWN_LEFT => !maze.is_wall_quad(row.inc(rows), col.dec(cols)),
            DIR_DOWN_RIGHT => !maze.is_wall_quad(row.inc(rows), col.inc(cols)),
            DIR_UP => !maze.is_wall_quad(row.dec(rows), col),
            DIR_UP_LEFT => !maze.is_wall_quad(row.dec(rows), col.dec(cols)),
            DIR_UP_RIGHT => !maze.is_wall_quad(row.dec(rows), col.inc(cols)),
            DIR_LEFT => !maze.is_wall_quad(row, col.dec(cols)),
            DIR_RIGHT => !maze.is_wall_quad(row, col.inc(cols)),
            _ => false,
        }
    }
}

fn update_player(player: &Player, maze: &Maze, frames: u32) -> Action {
    if frames < player.updated + PLAYER_FRAMES {
        return Action::Nothing;
    }
    let mut player = *player;
    if player.can_advance(maze, player.dir) {
        player.advance(player.dir, maze.dimensions);
    } else {
        if player.dir & DIR_UP != 0 && player.can_advance(maze, DIR_UP) {
            player.advance(DIR_UP, maze.dimensions);
        }
        if player.dir & DIR_DOWN != 0 && player.can_advance(maze, DIR_DOWN) {
            player.advance(DIR_DOWN, maze.dimensions);
        }
        if player.dir & DIR_LEFT != 0 && player.can_advance(maze, DIR_LEFT) {
            player.advance(DIR_LEFT, maze.dimensions);
        }
        if player.dir & DIR_RIGHT != 0 && player.can_advance(maze, DIR_RIGHT) {
            player.advance(DIR_RIGHT, maze.dimensions);
        }
    }
    Action::Update(Entity::Player(Player {
        updated: frames + PLAYER_FRAMES,
        cycle: (player.cycle + 1) & 0x3,
        ..player
    }))
}

fn update_bullet(bullet: &Bullet, maze: &Maze, frames: u32) -> Action {
    if frames < bullet.updated + BULLET_FRAMES {
        return Action::Nothing;
    }
    let (row, col) = (bullet.pos.row, bullet.pos.col);
    let (rows, cols) = (maze.rows(), maze.cols());
    let (row, col) = match bullet.dir {
        DIR_DOWN => (row.inc(rows), col),
        DIR_DOWN_LEFT => (row.inc(rows), col.dec(cols)),
        DIR_DOWN_RIGHT => (row.inc(rows), col.inc(cols)),
        DIR_UP => (row.dec(rows), col),
        DIR_UP_LEFT => (row.dec(rows), col.dec(cols)),
        DIR_UP_RIGHT => (row.dec(rows), col.inc(cols)),
        DIR_LEFT => (row, col.dec(cols)),
        DIR_RIGHT => (row, col.inc(cols)),
        _ => (row.inc(rows), col),
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
