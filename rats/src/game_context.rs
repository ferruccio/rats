use crate::{
    maze::{Maze, MAZE_CELL_COLS, MAZE_CELL_ROWS},
    player::{
        Direction, Player, DIR_DOWN, DIR_DOWN_LEFT, DIR_DOWN_RIGHT, DIR_LEFT,
        DIR_NONE, DIR_RIGHT, DIR_UP, DIR_UP_LEFT, DIR_UP_RIGHT,
    },
};
use sdl2::render::Texture;
use std::{cmp::max, time::Instant};
use video::{
    Chars, InitOptions, Result, Video, Wrapping, ATTR_DIM, ATTR_NONE,
    ATTR_REVERSE, BULLET_DOWN, BULLET_DOWN_LEFT, BULLET_DOWN_RIGHT,
    BULLET_LEFT, BULLET_RIGHT, BULLET_UP, BULLET_UP_LEFT, BULLET_UP_RIGHT,
};

pub struct GameContext {
    pub video: Video,
    pub start: Instant,
    pub frames: usize,
    pub maze: Maze,
    pub player: Player,
    pub direction: Direction,
    pub stop_direction: Direction,
    pub running: bool,
    pub firing: bool,
    pub player_motion_start: Instant,
    pub bullet_motion_start: Instant,
    pub bullet_fire_start: Instant,
    pub bullets: Vec<Bullet>,
}

#[derive(Debug)]
pub struct Bullet {
    pub row: Chars,
    pub col: Chars,
    pub direction: Direction,
}

impl GameContext {
    pub fn create(
        opts: InitOptions,
        maze_height: usize,
        maze_width: usize,
        density: usize,
    ) -> Result<GameContext> {
        let video = video::init(opts)?;
        let cell_rows = max((video.rows() - 2) / MAZE_CELL_ROWS, maze_height);
        let cell_cols = max(video.cols() / MAZE_CELL_COLS, maze_width);
        let mut the_maze = Maze::new(cell_rows, cell_cols);
        the_maze.generate(density);
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
            firing: false,
            player_motion_start: Instant::now(),
            bullet_motion_start: Instant::now(),
            bullet_fire_start: Instant::now(),
            bullets: vec![],
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

    pub fn fire(&mut self) {
        let row = self.player.position().row;
        let col = self.player.position().col;
        let rows = self.maze.rows();
        let cols = self.maze.cols();

        let direction = if self.direction == DIR_NONE {
            self.stop_direction
        } else {
            self.direction
        };
        let (row, col) = match direction {
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
        if self.maze.empty1(row, col) {
            self.bullets.push(Bullet {
                col,
                row,
                direction,
            });
        }
    }

    pub fn advance_bullets(&mut self) {
        let rows = self.maze.rows();
        let cols = self.maze.cols();
        let mut i = 0;
        loop {
            if i >= self.bullets.len() {
                return;
            }
            let bullet = &mut self.bullets[i];
            (bullet.row, bullet.col) = match bullet.direction {
                DIR_DOWN => (bullet.row.inc(rows), bullet.col),
                DIR_DOWN_LEFT => (bullet.row.inc(rows), bullet.col.dec(cols)),
                DIR_DOWN_RIGHT => (bullet.row.inc(rows), bullet.col.inc(cols)),
                DIR_UP => (bullet.row.dec(rows), bullet.col),
                DIR_UP_LEFT => (bullet.row.dec(rows), bullet.col.dec(cols)),
                DIR_UP_RIGHT => (bullet.row.dec(rows), bullet.col.inc(cols)),
                DIR_LEFT => (bullet.row, bullet.col.dec(cols)),
                DIR_RIGHT => (bullet.row, bullet.col.inc(cols)),
                _ => (bullet.row.inc(rows), bullet.col),
            };
            if !self.maze.empty1(bullet.row, bullet.col) {
                let last = self.bullets.len() - 1;
                if i != last {
                    self.bullets.swap(i, last);
                }
                self.bullets.truncate(last);
            }
            i += 1;
        }
    }

    pub fn render_bullets(&mut self, maze: &mut Maze) {
        for bullet in &self.bullets {
            let ch = match bullet.direction {
                DIR_DOWN => BULLET_DOWN,
                DIR_DOWN_LEFT => BULLET_DOWN_LEFT,
                DIR_DOWN_RIGHT => BULLET_DOWN_RIGHT,
                DIR_UP => BULLET_UP,
                DIR_UP_LEFT => BULLET_UP_LEFT,
                DIR_UP_RIGHT => BULLET_UP_RIGHT,
                DIR_LEFT => BULLET_LEFT,
                DIR_RIGHT => BULLET_RIGHT,
                _ => BULLET_DOWN,
            };
            maze.buffer
                .set_chattr(bullet.row, bullet.col, ch, ATTR_NONE);
        }
    }
}
