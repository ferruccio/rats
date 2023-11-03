use clap::Parser;
use entities::dir;
use game_context::GameContext;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use video::{
    sdl_error, Event, InitOptions, Keycode, PixelFormatEnum, Pixels, Result,
    Size, ATTR_COMBOS, CHAR_CELL_HEIGHT, CHAR_CELL_WIDTH, FONT_SIZE,
};

mod entities;
mod game_context;
mod maze;

#[derive(Parser)]
struct CommandLineOpts {
    /// Display index
    #[clap(short = 'd', long = "display")]
    display: Option<usize>,

    /// Number of rat factories
    #[clap(short = 'f', long = "rat-factories", default_value_t = 5)]
    factories: usize,

    /// Maze height (maze cells)
    #[clap(short = 'H', long = "maze-height", alias = "mh")]
    maze_height: Option<usize>,

    /// Maze width (maze cells)
    #[clap(short = 'W', long = "maze-width", alias = "mw")]
    maze_width: Option<usize>,

    /// Maze density
    #[clap(short = 'm', long = "maze-density", default_value_t = 85)]
    density: usize,

    /// Window height (pixels)
    #[clap(long = "window-height", alias = "wh")]
    window_height: Option<Pixels>,

    /// Window width (pixels)
    #[clap(long = "window-width", alias = "ww")]
    window_width: Option<Pixels>,

    /// Scale factor (1 to 4)
    #[clap(short = 's', long = "scale")]
    scale: Option<usize>,
}

fn main() {
    if let Err(error) = play(CommandLineOpts::parse()) {
        println!("{error}");
    }
}

const FPS_LIMIT: u32 = 60;
const RAT_SPAWN_SECONDS: u64 = 15;

fn play(opts: CommandLineOpts) -> Result<()> {
    let cell_rows = opts.maze_height.unwrap_or(15);
    let cell_cols = opts.maze_width.unwrap_or(15);
    let mut context = GameContext::create(
        InitOptions::new()
            .display_index(opts.display)
            .window_height(opts.window_height)
            .window_width(opts.window_width)
            .scale(opts.scale),
        cell_rows as Size,
        cell_cols as Size,
        opts.density,
        opts.factories,
    )?;

    let texture_creator = context.video.canvas.texture_creator();
    let mut textures = vec![];
    for _ in 0..ATTR_COMBOS {
        let texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24,
            (CHAR_CELL_WIDTH * context.video.scale) as u32,
            (FONT_SIZE as usize * CHAR_CELL_HEIGHT * context.video.scale)
                as u32,
        )?;
        textures.push(texture);
    }
    context
        .video
        .init_charmap_textures(&mut textures, context.video.scale)?;

    const NANOS_PER_FRAME: u32 = 1_000_000_000 / FPS_LIMIT;
    let mut frame_time = Instant::now();
    let mut event_pump = context.video.sdl.event_pump().map_err(sdl_error)?;
    // player can fire 4 bullets/second
    let bullet_firing_time = Duration::new(0, 1_000_000_000 / 4);
    let mut rat_spawn_time =
        Instant::now() - Duration::new(RAT_SPAWN_SECONDS, 0);
    let mut running = true;
    while running {
        context.render_frame(&textures)?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => running = key_down(&mut context, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => key_up(&mut context, keycode),
                _ => {}
            }
        }
        context.update();
        if context.firing
            && context.bullet_fire_start.elapsed() >= bullet_firing_time
        {
            context.fire();
            context.bullet_fire_start = Instant::now();
        }
        if rat_spawn_time.elapsed().as_secs() >= RAT_SPAWN_SECONDS {
            // New rats will always be generated at the same rate regardless of
            // how many factories are left. This means that as factories are
            // destroyed, remaining factories will pick up the pace to
            // compensate for the loss of our rat making comrades.
            context.new_rats = opts.factories;
            rat_spawn_time = Instant::now();
        }

        // don't hog the CPU
        let nanos_elapsed = frame_time.elapsed().as_nanos() as u32;
        if nanos_elapsed < NANOS_PER_FRAME {
            sleep(Duration::new(0, NANOS_PER_FRAME - nanos_elapsed));
        }
        frame_time = Instant::now();
    }

    Ok(())
}

// return true to keep game running
fn key_down(context: &mut GameContext, keycode: Keycode) -> bool {
    match keycode {
        Keycode::Escape | Keycode::Q => return false,
        Keycode::Up => context.start(dir::UP),
        Keycode::Down => context.start(dir::DOWN),
        Keycode::Left => context.start(dir::LEFT),
        Keycode::Right => context.start(dir::RIGHT),
        Keycode::Space => {
            if !context.firing {
                context.fire();
                context.bullet_fire_start = Instant::now();
            }
            context.firing = true;
        }
        _ => {}
    }
    true
}

fn key_up(context: &mut GameContext, keycode: Keycode) {
    match keycode {
        Keycode::Up => context.stop(dir::UP),
        Keycode::Down => context.stop(dir::DOWN),
        Keycode::Left => context.stop(dir::LEFT),
        Keycode::Right => context.stop(dir::RIGHT),
        Keycode::Space => context.firing = false,
        _ => {}
    }
}
