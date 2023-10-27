use clap::Parser;
use maze::{Maze, MAZE_CELL_COLS, MAZE_CELL_ROWS};
use std::{cmp::max, time::Instant};
use video::{
    Chars, Event, InitOptions, Keycode, Pixels, Result, Video, ATTR_DIM,
    ATTR_REVERSE,
};

use crate::player::{
    Direction, Player, DIR_DOWN, DIR_LEFT, DIR_NONE, DIR_RIGHT, DIR_UP,
};

mod maze;
mod player;

#[derive(Parser)]
struct CommandLineParams {
    /// Display index
    #[clap(short = 'd', long = "display")]
    display: Option<usize>,

    /// Window height (pixels)
    #[clap(long = "window_height", alias = "w-ht")]
    window_height: Option<Pixels>,

    /// Window width (pixels)
    #[clap(long = "window-width", alias = "w-wt")]
    window_width: Option<Pixels>,

    /// Maze height (characters)
    #[clap(long = "maze-height", alias = "m-ht")]
    maze_height: Option<Chars>,

    /// Maze width (characters)
    #[clap(long = "maze-width", alias = "m-wt")]
    maze_width: Option<Chars>,

    /// Scale factor (1 to 4)
    #[clap(short = 's', long = "scale")]
    scale: Option<usize>,
}

fn main() {
    if let Err(error) = play(CommandLineParams::parse()) {
        println!("{error}");
    }
}

fn play(opts: CommandLineParams) -> Result<()> {
    let mut video = video::init(
        InitOptions::new()
            .display_index(opts.display)
            .window_height(opts.window_height)
            .window_width(opts.window_width)
            .scale(opts.scale),
    )?;
    video.init_charmap()?;

    let (cell_rows, cell_cols) = maze_dimensions(&opts, &video);
    let mut the_maze = Maze::new(cell_rows, cell_cols);
    the_maze.test_pattern();

    let mut player = Player::new(&the_maze);
    for _ in 0..MAZE_CELL_COLS / 2 {
        player.advance(DIR_RIGHT);
    }
    for _ in 0..MAZE_CELL_ROWS / 2 {
        player.advance(DIR_DOWN);
    }
    dbg!(&player);
    let mut direction: Direction = DIR_NONE;

    let mut running = true;
    let mut frames = 0;
    let mut move_player = false;
    let mut maze = Maze::new(cell_rows, cell_cols);
    let start = Instant::now();
    while running {
        the_maze.buffer.copy_to(&mut maze.buffer);
        player.render(&mut maze);

        let mut start_pos = player.position();
        start_pos.move_up((video.buffer.rows - 2) / 2);
        start_pos.move_left(video.buffer.cols / 2);

        maze.buffer.copy_buffer(
            start_pos.row,
            start_pos.col,
            &mut video.buffer,
            2,
        );
        frames += 1;

        let seconds = start.elapsed().as_secs_f32();
        let fps = frames as f32 / if seconds == 0.0 { 1.0 } else { seconds };
        video.buffer.print(
            0,
            0,
            ATTR_REVERSE | ATTR_DIM,
            format!("FPS: {fps:.0} dir: {direction:02X} start: {start_pos}"),
        );
        video.buffer.print(
            1,
            0,
            ATTR_REVERSE | ATTR_DIM,
            format!(
                "maze: {cell_rows}R x {cell_cols}C ({rows} x {cols}) player: {player}",
                rows = maze.cols(),
                cols = maze.rows(),
                player = player.position()
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

fn maze_dimensions(opts: &CommandLineParams, video: &Video) -> (Chars, Chars) {
    (
        // rows
        max(
            (video.rows() - 2) / MAZE_CELL_ROWS,
            opts.maze_height.unwrap_or(15),
        ),
        // cols
        max(video.cols() / MAZE_CELL_COLS, opts.maze_width.unwrap_or(15)),
    )
}
