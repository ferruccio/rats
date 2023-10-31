use clap::Parser;
use game_context::GameContext;
use maze::Maze;
use player::{DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use video::{
    sdl_error, Event, InitOptions, Keycode, PixelFormatEnum, Pixels, Result,
    ATTR_COMBOS, CHAR_CELL_HEIGHT, CHAR_CELL_WIDTH, FONT_SIZE,
};

use crate::player::DIR_NONE;

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
    #[clap(short = 's', long = "scale", default_value_t = 2)]
    scale: usize,

    /// Maze density
    #[clap(short = 'm', long = "maze-density", default_value_t = 85)]
    density: usize,
}

fn main() {
    if let Err(error) = play(CommandLineParams::parse()) {
        println!("{error}");
    }
}

fn play(opts: CommandLineParams) -> Result<()> {
    let cell_rows = opts.maze_height.unwrap_or(15);
    let cell_cols = opts.maze_width.unwrap_or(15);
    let scale = opts.scale.clamp(1, 4);
    let mut context = GameContext::create(
        InitOptions::new()
            .display_index(opts.display)
            .window_height(opts.window_height)
            .window_width(opts.window_width)
            .scale(opts.scale),
        cell_rows,
        cell_cols,
        opts.density,
    )?;

    let texture_creator = context.video.canvas.texture_creator();
    let mut textures = vec![];
    for _ in 0..ATTR_COMBOS {
        let texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24,
            (CHAR_CELL_WIDTH * scale) as u32,
            (FONT_SIZE * CHAR_CELL_HEIGHT * scale) as u32,
        )?;
        textures.push(texture);
    }
    context.video.init_charmap_textures(&mut textures, scale)?;

    const FPS_LIMIT: u32 = 60;
    const NANOS_PER_FRAME: u32 = 1_000_000_000 / FPS_LIMIT;
    let mut frame_time = Instant::now();

    let mut maze = Maze::new(cell_rows, cell_cols);
    let mut event_pump = context.video.sdl.event_pump().map_err(sdl_error)?;
    // player moves every 1/10th of a second
    let motion_time = Duration::new(0, 1_000_000_000 / 10);
    let mut motion_cycle: u8 = 0;
    while context.running {
        context.maze.buffer.copy_to(&mut maze.buffer);
        let offset = if context.direction == DIR_NONE {
            0
        } else {
            (motion_cycle >> 1) + 1
        };
        context.render_frame(&mut maze, offset, &textures)?;
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
        if context.motion_start.elapsed() >= motion_time {
            context.player.advance_all(&maze, context.direction);
            context.motion_start = Instant::now();
            motion_cycle = (motion_cycle + 1) & 3;
        }

        let nanos_elapsed = frame_time.elapsed().as_nanos() as u32;
        if nanos_elapsed < NANOS_PER_FRAME {
            sleep(Duration::new(0, NANOS_PER_FRAME - nanos_elapsed));
        }
        frame_time = Instant::now();
    }

    Ok(())
}

fn key_down(context: &mut GameContext, keycode: Keycode) {
    match keycode {
        Keycode::Escape | Keycode::Q => context.running = false,
        Keycode::Up => context.start(DIR_UP),
        Keycode::Down => context.start(DIR_DOWN),
        Keycode::Left => context.start(DIR_LEFT),
        Keycode::Right => context.start(DIR_RIGHT),
        _ => {}
    }
}

fn key_up(context: &mut GameContext, keycode: Keycode) {
    match keycode {
        Keycode::Up => context.stop(DIR_UP),
        Keycode::Down => context.stop(DIR_DOWN),
        Keycode::Left => context.stop(DIR_LEFT),
        Keycode::Right => context.stop(DIR_RIGHT),
        _ => {}
    }
}
