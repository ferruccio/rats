use clap::Parser;
use maze::{Maze, MAZE_CELL_HEIGHT, MAZE_CELL_WIDTH};
use std::{
    cmp::{max, min},
    time::Instant,
};
use video::{
    Event, InitOptions, Keycode, Result, Video, ATTR_DIM, ATTR_REVERSE,
};

use crate::player::{
    Direction, Player, DIR_DOWN, DIR_LEFT, DIR_NONE, DIR_RIGHT, DIR_UP,
};

mod maze;
mod player;

#[derive(Parser)]
struct RatsOpts {
    /// Display index
    #[clap(short = 'd', long = "display")]
    display: Option<usize>,

    /// Window height (pixels)
    #[clap(long = "window_height", alias = "w-ht")]
    window_height: Option<usize>,

    /// Window width (pixels)
    #[clap(long = "window-width", alias = "w-wt")]
    window_width: Option<usize>,

    /// Maze height (characters)
    #[clap(long = "maze-height", alias = "m-ht")]
    maze_height: Option<usize>,

    /// Maze width (characters)
    #[clap(long = "maze-width", alias = "m-wt")]
    maze_width: Option<usize>,

    /// Scale factor (1 to 4)
    #[clap(short = 's', long = "scale")]
    scale: Option<usize>,
}

fn main() {
    if let Err(error) = play(RatsOpts::parse()) {
        println!("{error}");
    }
}

fn play(opts: RatsOpts) -> Result<()> {
    let mut video = video::init(
        InitOptions::new()
            .display_index(opts.display)
            .window_height(opts.window_height)
            .window_width(opts.window_width)
            .scale(opts.scale),
    )?;
    dbg!(&video.buffer);
    video.init_charmap()?;

    let (maze_width, maze_height) = maze_dimensions(&opts, &video);
    let mut the_maze = Maze::new(maze_height, maze_width);
    dbg!(&the_maze);
    the_maze.test_pattern();

    let mut player = Player::new(&the_maze);
    let mut direction: Direction = DIR_NONE;

    let mut running = true;
    let mut frames = 0;
    let mut move_player = false;
    let mut maze = Maze::new(maze_height, maze_width);
    let start = Instant::now();
    while running {
        the_maze.buffer.copy_to(&mut maze.buffer);
        player.render(&mut maze);

        let start_row = if player.row() >= video.buffer.rows / 2 {
            player.row() - video.buffer.rows / 2
        } else {
            maze.buffer.rows - video.buffer.rows / 2
        };
        let start_col = if player.col() >= video.buffer.cols / 2 {
            player.col() - video.buffer.cols / 2
        } else {
            maze.buffer.cols - video.buffer.cols / 2
        };

        maze.buffer
            .copy_buffer(start_row, start_col, &mut video.buffer, 2);
        frames += 1;

        let seconds = start.elapsed().as_secs_f32();
        let fps = frames as f32 / if seconds == 0.0 { 1.0 } else { seconds };
        video.buffer.print(
            0,
            0,
            ATTR_REVERSE | ATTR_DIM,
            format!("FPS: {fps:.0} dir: {direction:02X} sr: {start_row} sc: {start_col}"));
        video.buffer.print(
            1,
            0,
            ATTR_REVERSE | ATTR_DIM,
            format!(
                "maze: {maze_width}W x {maze_height}H ({wt} x {ht}) player r:{row} c:{col}",
                wt = maze.buffer.cols,
                ht = maze.buffer.rows,
                row = player.row(),
                col = player.col()
            ),
        );

        video.render_buffer()?;

        video.handle_events(|event| match event {
            Event::Quit { .. } => running = false,
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Escape | Keycode::Q => running = false,
                Keycode::Up => {
                    direction |= DIR_UP;
                    move_player = true;
                }
                Keycode::Down => {
                    direction |= DIR_DOWN;
                    move_player = true;
                }
                Keycode::Left => {
                    direction |= DIR_LEFT;
                    move_player = true;
                }
                Keycode::Right => {
                    direction |= DIR_RIGHT;
                    move_player = true;
                }
                _ => {}
            },
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Up => direction &= !DIR_UP,
                Keycode::Down => direction &= !DIR_DOWN,
                Keycode::Left => direction &= !DIR_LEFT,
                Keycode::Right => direction &= !DIR_RIGHT,
                _ => {}
            },
            _ => {}
        });
        if move_player {
            player.advance(direction);
            move_player = false;
        }
    }

    Ok(())
}

fn maze_dimensions(opts: &RatsOpts, video: &Video) -> (usize, usize) {
    (
        // height
        if let Some(height) = opts.maze_height {
            min(height, 30)
        } else {
            max(video.rows() / (MAZE_CELL_HEIGHT), 30)
        }, //width
        if let Some(width) = opts.maze_width {
            min(width, 15)
        } else {
            max(video.cols() / (MAZE_CELL_WIDTH + 1), 15)
        },
    )
}
