use crate::{
    entities::{
        Bullet, Direction, Entity, EntityList, Player, Position, State,
        DIR_DOWN, DIR_DOWN_LEFT, DIR_DOWN_RIGHT, DIR_LEFT, DIR_NONE, DIR_RIGHT,
        DIR_UP, DIR_UP_LEFT, DIR_UP_RIGHT,
    },
    maze::{Maze, MAZE_CELL_COLS, MAZE_CELL_ROWS},
};
use std::{cmp::max, time::Instant};
use video::{InitOptions, Pos, Result, Size, SizeWrapping, Video};

mod render;
mod update;

pub use render::*;
pub use update::*;

pub struct GameContext {
    pub video: Video,
    pub start: Instant,
    pub frames: u32,
    pub pristine_maze: Maze,
    pub maze: Maze,
    pub running: bool,
    pub firing: bool,
    pub player_motion_start: Instant,
    pub bullet_motion_start: Instant,
    pub bullet_fire_start: Instant,
    pub entities: EntityList,
}

impl GameContext {
    pub fn create(
        opts: InitOptions,
        maze_height: Size,
        maze_width: Size,
        density: usize,
    ) -> Result<GameContext> {
        let video = video::init(opts)?;
        let maze_rows = max((video.rows() - 2) / MAZE_CELL_ROWS, maze_height);
        let maze_cols = max(video.cols() / MAZE_CELL_COLS, maze_width);
        let mut pristine_maze = Maze::new(maze_rows, maze_cols);
        pristine_maze.generate(density);
        let mut context = GameContext {
            video,
            start: Instant::now(),
            frames: 0,
            pristine_maze,
            maze: Maze::new(maze_rows, maze_cols),
            running: true,
            firing: false,
            player_motion_start: Instant::now(),
            bullet_motion_start: Instant::now(),
            bullet_fire_start: Instant::now(),
            entities: EntityList::new(),
        };
        context.entities.push(Entity::Player(Player {
            updated: context.frames,
            pos: Position {
                row: (MAZE_CELL_ROWS / 2) as Pos,
                col: (MAZE_CELL_COLS / 2) as Pos,
            },
            dir: DIR_NONE,
            stop_dir: DIR_DOWN,
            state: State::Alive,
            cycle: 0,
        }));
        Ok(context)
    }

    pub fn player_position(&self) -> Position {
        self.get_player().pos.clone()
    }

    pub fn get_player(&self) -> &Player {
        match self.entities.get(0).expect("get_player: can't get player") {
            Entity::Player(player) => &player,
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
        player.stop_dir = match dir {
            DIR_UP | DIR_DOWN | DIR_LEFT | DIR_RIGHT => dir,
            DIR_UP_LEFT | DIR_DOWN_LEFT => DIR_LEFT,
            DIR_UP_RIGHT | DIR_DOWN_RIGHT => DIR_RIGHT,
            _ => DIR_UP,
        };
    }

    pub fn stop(&mut self, dir: Direction) {
        let player = self.get_player_mut();
        player.dir = player.dir & !dir;
    }

    pub fn fire(&mut self) {
        let player = self.get_player();
        let dir = if player.dir == DIR_NONE {
            player.stop_dir
        } else {
            player.dir
        };
        let (row, col) = (player.pos.row, player.pos.col);
        let (rows, cols) = (self.maze.rows(), self.maze.cols());

        let (row, col) = match dir {
            DIR_DOWN => (row.inc(rows).inc(rows), col),
            DIR_DOWN_LEFT => (row.inc(rows), col.dec(cols)),
            DIR_DOWN_RIGHT => {
                (row.inc(rows).inc(rows), col.inc(cols).inc(cols))
            }
            DIR_UP => (row.dec(rows), col.inc(cols)),
            DIR_UP_LEFT => (row.dec(rows), col.dec(cols)),
            DIR_UP_RIGHT => (row.dec(rows), col.inc(cols)),
            DIR_LEFT => (row, col.dec(cols)),
            DIR_RIGHT => (row, col.inc(cols).inc(cols)),
            _ => (row, col),
        };
        if !self.maze.is_wall(row, col) {
            self.entities.push(Entity::Bullet(Bullet {
                updated: self.frames,
                pos: Position { row, col },
                dir,
                state: State::Alive,
            }));
        }
    }
}
