use clap::Parser;
use game_context::GameContext;
use maze::Maze;
use player::{DIR_DOWN, DIR_LEFT, DIR_NONE, DIR_RIGHT, DIR_UP};
use std::time::{Duration, Instant};
use video::{sdl_error, Event, InitOptions, Keycode, Pixels, Result};

mod game_context;
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

    /// Maze height (maze cells)
    #[clap(long = "maze-height", alias = "m-ht")]
    maze_height: Option<usize>,

    /// Maze width (maze cells)
    #[clap(long = "maze-width", alias = "m-wt")]
    maze_width: Option<usize>,

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
    let cell_rows = opts.maze_height.unwrap_or(15);
    let cell_cols = opts.maze_width.unwrap_or(15);
    let mut context = GameContext::create(
        InitOptions::new()
            .display_index(opts.display)
            .window_height(opts.window_height)
            .window_width(opts.window_width)
            .scale(opts.scale),
        cell_rows,
        cell_cols,
    )?;

    let mut maze = Maze::new(cell_rows, cell_cols);
    let mut event_pump = context.video.sdl.event_pump().map_err(sdl_error)?;
    // player moves every 1/10th of a second
    let motion_time = Duration::new(0, 1_000_000_000 / 10);
    while context.running {
        context.maze.buffer.copy_to(&mut maze.buffer);
        context.render_frame(&mut maze)?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => context.running = false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => key_down(&mut context, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => key_up(&mut context, keycode),
                _ => {}
            }
        }
        if context.direction != DIR_NONE
            && context.motion_start.elapsed() >= motion_time
        {
            context.player.advance(context.direction);
            context.motion_start = Instant::now();
        }
    }

    Ok(())
}

fn key_down(context: &mut GameContext, keycode: Keycode) {
    match keycode {
        Keycode::Escape | Keycode::Q => context.running = false,
        Keycode::Up => context.direction |= DIR_UP,
        Keycode::Down => context.direction |= DIR_DOWN,
        Keycode::Left => context.direction |= DIR_LEFT,
        Keycode::Right => context.direction |= DIR_RIGHT,
        _ => {}
    }
}

fn key_up(context: &mut GameContext, keycode: Keycode) {
    match keycode {
        Keycode::Up => context.direction &= !DIR_UP,
        Keycode::Down => context.direction &= !DIR_DOWN,
        Keycode::Left => context.direction &= !DIR_LEFT,
        Keycode::Right => context.direction &= !DIR_RIGHT,
        _ => {}
    }
}
