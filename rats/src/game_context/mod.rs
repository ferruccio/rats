use crate::{
    entities::{
        dir, Bullet, Direction, Entity, EntityList, Player, Position, State,
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
            dir: dir::NONE,
            stop_dir: dir::DOWN,
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
            dir::UP | dir::DOWN | dir::LEFT | dir::RIGHT => dir,
            dir::UP_LEFT | dir::DOWN_LEFT => dir::LEFT,
            dir::UP_RIGHT | dir::DOWN_RIGHT => dir::RIGHT,
            _ => dir::UP,
        };
    }

    pub fn stop(&mut self, dir: Direction) {
        let player = self.get_player_mut();
        player.dir = player.dir & !dir;
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

        let (row, col) = match dir {
            dir::DOWN => (row.inc(rows).inc(rows), col),
            dir::DOWN_LEFT => (row.inc(rows), col.dec(cols)),
            dir::DOWN_RIGHT => {
                (row.inc(rows).inc(rows), col.inc(cols).inc(cols))
            }
            dir::UP => (row.dec(rows), col.inc(cols)),
            dir::UP_LEFT => (row.dec(rows), col.dec(cols)),
            dir::UP_RIGHT => (row.dec(rows), col.inc(cols)),
            dir::LEFT => (row, col.dec(cols)),
            dir::RIGHT => (row, col.inc(cols).inc(cols)),
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
