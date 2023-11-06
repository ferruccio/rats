use crate::{
    entities::{
        dir, state, Direction, Entity, EntityList, Player, Position,
        PLAYER_FIRE_RATE_NS,
    },
    maze::{Maze, MAZE_CELL_COLS, MAZE_CELL_ROWS},
};
use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    thread_rng, Rng,
};
use std::{
    cmp::max,
    fmt::Display,
    time::{Duration, Instant},
};
use video::{InitOptions, Pos, Result, Size, Video};

mod factories;
mod firing;
mod render;
mod update;

pub use factories::*;
pub use firing::*;
pub use render::*;
pub use update::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    RUNNING,
    PAUSED,
    FINISHED,
    QUIT,
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(match self {
            GameState::RUNNING => write!(f, "RUNNING")?,
            GameState::PAUSED => write!(f, "PAUSED")?,
            GameState::FINISHED => write!(f, "FINISHED")?,
            GameState::QUIT => write!(f, "QUIT")?,
        })
    }
}

pub struct GameContext {
    pub game_state: GameState,
    pub diagnostics: bool,
    pub video: Video,
    pub start: Instant,
    pub frames: u32,
    pub pristine_maze: Maze,
    pub maze: Maze,
    pub firing_dir: Direction,
    pub bullet_fire_start: Instant,
    pub bullet_firing_time: Duration,
    pub next_fire_time: Instant,
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
            game_state: GameState::RUNNING,
            diagnostics: false,
            video,
            start: Instant::now(),
            frames: 0,
            pristine_maze: pristine_maze.clone(),
            maze: Maze::new(maze_rows, maze_cols),
            firing_dir: dir::NONE,
            bullet_fire_start: Instant::now(),
            bullet_firing_time: Duration::new(0, PLAYER_FIRE_RATE_NS),
            next_fire_time: Instant::now(),
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
            update: context.elapsed(),
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

    pub fn elapsed(&self) -> u32 {
        self.start.elapsed().as_millis() as u32
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
