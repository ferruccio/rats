use crate::{
    entities::{
        dir, state, Bullet, Direction, Entity, EntityList, Player, Position,
    },
    maze::{Maze, MAZE_CELL_COLS, MAZE_CELL_ROWS},
};
use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    thread_rng, Rng,
};
use std::{cmp::max, time::Instant};
use video::{InitOptions, Pos, Result, Size, SizeWrapping, Video};

mod factories;
mod render;
mod update;

pub use factories::*;
pub use render::*;
pub use update::*;

pub struct GameContext {
    pub diagnostics: bool,
    pub video: Video,
    pub start: Instant,
    pub frames: u32,
    pub pristine_maze: Maze,
    pub maze: Maze,
    pub firing: bool,
    pub bullet_fire_start: Instant,
    pub entities: EntityList,
    pub live_factories: usize,
    pub dead_factories: usize,
    pub live_rats: usize,
    pub dead_rats: usize,
    pub live_brats: usize,
    pub dead_brats: usize,
    pub new_rats: usize,
    pub new_brats: usize,
    pub super_boom: usize,
    pub score: usize,
}

impl GameContext {
    pub fn create(
        opts: InitOptions,
        maze_height: Size,
        maze_width: Size,
        density: usize,
        factories: usize,
    ) -> Result<GameContext> {
        let video = video::init(opts)?;
        let maze_rows = max((video.rows() - 2) / MAZE_CELL_ROWS, maze_height);
        let maze_cols = max(video.cols() / MAZE_CELL_COLS, maze_width);
        let mut pristine_maze = Maze::new(maze_rows, maze_cols);
        pristine_maze.generate(density);
        let mut context = GameContext {
            diagnostics: false,
            video,
            start: Instant::now(),
            frames: 0,
            pristine_maze: pristine_maze.clone(),
            maze: Maze::new(maze_rows, maze_cols),
            firing: false,
            bullet_fire_start: Instant::now(),
            entities: EntityList::new(),
            live_factories: 0,
            dead_factories: 0,
            live_rats: 0,
            dead_rats: 0,
            live_brats: 0,
            dead_brats: 0,
            new_rats: 0,
            new_brats: 0,
            super_boom: 0,
            score: 0,
        };
        context.entities.push(Entity::Player(Player {
            update: context.frames,
            pos: Position {
                row: (MAZE_CELL_ROWS / 2) as Pos,
                col: (MAZE_CELL_COLS / 2) as Pos,
            },
            dir: dir::NONE,
            stop_dir: dir::DOWN,
            state: state::ALIVE,
            cycle: 0,
        }));
        context.generate_factories(factories.clamp(1, 100), &pristine_maze);
        Ok(context)
    }

    pub fn player_position(&self) -> Position {
        self.get_player().pos
    }

    pub fn get_player(&self) -> &Player {
        match self.entities.get(0).expect("get_player: can't get player") {
            Entity::Player(player) => player,
            _ => panic!("get_player: player is not a Player"),
        }
    }

    pub fn get_player_mut(&mut self) -> &mut Player {
        match self
            .entities
            .get_mut(0)
            .expect("get_player_mut: can't get player")
        {
            Entity::Player(player) => player,
            _ => panic!("get_player_mut: player is not a Player"),
        }
    }

    pub fn start(&mut self, dir: Direction) {
        let player = self.get_player_mut();
        player.dir |= dir;
        player.stop_dir = dir::stop_dir(dir);
    }

    pub fn stop(&mut self, dir: Direction) {
        let player = self.get_player_mut();
        player.dir = if dir == dir::NONE {
            dir
        } else {
            player.dir & !dir
        };
    }

    pub fn fire(&mut self) {
        let player = self.get_player();
        let dir = if player.dir == dir::NONE {
            player.stop_dir
        } else {
            player.dir
        };
        let (row, col) = (player.pos.row, player.pos.col);
        let (rows, cols) = (self.maze.rows(), self.maze.cols());

        if let Some((row, col)) = match dir {
            dir::DOWN => Some((row.inc(rows).inc(rows), col)),
            dir::DOWN_LEFT => Some((row.inc(rows), col.dec(cols))),
            dir::DOWN_RIGHT => {
                Some((row.inc(rows).inc(rows), col.inc(cols).inc(cols)))
            }
            dir::UP => Some((row.dec(rows), col.inc(cols))),
            dir::UP_LEFT => Some((row.dec(rows), col.dec(cols))),
            dir::UP_RIGHT => Some((row.dec(rows), col.inc(cols))),
            dir::LEFT => Some((row, col.dec(cols))),
            dir::RIGHT => Some((row, col.inc(cols).inc(cols))),
            _ => None,
        } {
            if !self.maze.is_wall(row, col) {
                self.entities.push(Entity::Bullet(Bullet {
                    update: self.frames,
                    pos: Position { row, col },
                    dir,
                    state: state::ALIVE,
                }));
            }
        }
    }
}

pub fn random<T: SampleUniform>(low: T, high: T) -> T {
    let mut rng = thread_rng();
    let distribution = Uniform::new_inclusive(low, high);
    rng.sample(distribution)
}

pub fn random_direction() -> Direction {
    match random(0, 3) {
        0 => dir::UP,
        1 => dir::DOWN,
        2 => dir::LEFT,
        _ => dir::RIGHT,
    }
}

pub fn flip_a_coin() -> bool {
    random(0, 99) > 50
}
