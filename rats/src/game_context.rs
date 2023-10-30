use crate::{
    maze::{Maze, MAZE_CELL_COLS, MAZE_CELL_ROWS},
    player::{
        Direction, Player, DIR_DOWN, DIR_DOWN_LEFT, DIR_DOWN_RIGHT, DIR_LEFT,
        DIR_NONE, DIR_RIGHT, DIR_UP, DIR_UP_LEFT, DIR_UP_RIGHT,
    },
};
use sdl2::render::Texture;
use std::{cmp::max, time::Instant};
use video::{InitOptions, Result, Video, ATTR_DIM, ATTR_REVERSE};

pub struct GameContext {
    pub video: Video,
    pub start: Instant,
    pub frames: usize,
    pub maze: Maze,
    pub player: Player,
    pub direction: Direction,
    pub stop_direction: Direction,
    pub running: bool,
    pub motion_start: Instant,
}

impl GameContext {
    pub fn create(
        opts: InitOptions,
        maze_height: usize,
        maze_width: usize,
    ) -> Result<GameContext> {
        let video = video::init(opts)?;
        let cell_rows = max((video.rows() - 2) / MAZE_CELL_ROWS, maze_height);
        let cell_cols = max(video.cols() / MAZE_CELL_COLS, maze_width);
        let mut the_maze = Maze::new(cell_rows, cell_cols);
        the_maze.generate(false);
        let mut player = Player::new(&the_maze);
        for _ in 0..MAZE_CELL_COLS / 2 {
            player.advance(DIR_RIGHT);
        }
        for _ in 0..MAZE_CELL_ROWS / 2 {
            player.advance(DIR_DOWN);
        }
        Ok(GameContext {
            video,
            start: Instant::now(),
            frames: 0,
            maze: the_maze,
            player,
            direction: DIR_NONE,
            stop_direction: DIR_DOWN,
            running: true,
            motion_start: Instant::now(),
        })
    }

    pub fn render_frame(
        &mut self,
        maze: &mut Maze,
        offset: u8,
        textures: &[Texture],
    ) -> Result<()> {
        self.video.buffer.clear();
        self.player.render(
            maze,
            if self.direction == DIR_NONE {
                self.stop_direction
            } else {
                self.direction
            },
            offset,
        );

        let mut start_pos = self.player.position();
        start_pos.move_up((self.video.buffer.rows - 2) / 2);
        start_pos.move_left(self.video.buffer.cols / 2);

        maze.buffer.copy_buffer(
            start_pos.row,
            start_pos.col,
            &mut self.video.buffer,
            2,
        );
        self.frames += 1;

        let seconds = self.start.elapsed().as_secs_f32();
        let fps =
            self.frames as f32 / if seconds == 0.0 { 1.0 } else { seconds };
        self.video.buffer.print(
            0,
            0,
            ATTR_REVERSE | ATTR_DIM,
            format!("FPS: {fps:.0} start: {start_pos}"),
        );
        self.video.buffer.print(
            1,
            0,
            ATTR_REVERSE | ATTR_DIM,
            format!(
                "maze: {rows}x{cols} player: {player}",
                rows = maze.rows(),
                cols = maze.cols(),
                player = self.player.position()
            ),
        );

        self.video.render_buffer(textures)
    }

    pub fn start(&mut self, dir: Direction) {
        self.direction |= dir;
        self.stop_direction = match self.direction {
            DIR_UP | DIR_DOWN | DIR_LEFT | DIR_RIGHT => self.direction,
            DIR_UP_LEFT | DIR_DOWN_LEFT => DIR_LEFT,
            DIR_UP_RIGHT | DIR_DOWN_RIGHT => DIR_RIGHT,
            _ => DIR_UP,
        };
    }

    pub fn stop(&mut self, dir: Direction) {
        self.direction = self.direction & !dir;
    }
}
