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
struct CommandLineParams {
    /// Display index
    #[clap(short = 'd', long = "display")]
    display: Option<usize>,

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
    #[clap(long = "window_height", alias = "wh")]
    window_height: Option<Pixels>,

    /// Window width (pixels)
    #[clap(long = "window-width", alias = "ww")]
    window_width: Option<Pixels>,

    /// Scale factor (1 to 4)
    #[clap(short = 's', long = "scale", default_value_t = 2)]
    scale: usize,
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
        cell_rows as Size,
        cell_cols as Size,
        opts.density,
    )?;

    let texture_creator = context.video.canvas.texture_creator();
    let mut textures = vec![];
    for _ in 0..ATTR_COMBOS {
        let texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24,
            (CHAR_CELL_WIDTH * scale) as u32,
            (FONT_SIZE as usize * CHAR_CELL_HEIGHT * scale) as u32,
        )?;
        textures.push(texture);
    }
    context.video.init_charmap_textures(&mut textures, scale)?;

    const FPS_LIMIT: u32 = 60;
    const NANOS_PER_FRAME: u32 = 1_000_000_000 / FPS_LIMIT;
    let mut frame_time = Instant::now();

    let mut event_pump = context.video.sdl.event_pump().map_err(sdl_error)?;
    // // player moves every 1/10th of a second
    // let player_motion_time = Duration::new(0, 1_000_000_000 / 10);
    // // bullets move every 1/20th of a second
    let bullet_motion_time = Duration::new(0, 1_000_000_000 / 20);
    // // player can fire every 1/4 of a second
    let bullet_firing_time = Duration::new(0, 1_000_000_000 / 4);
    // let mut motion_cycle: u8 = 0;
    while context.running {
        context.render_frame(&textures)?;
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
        context.update();

        /*
        if context.player_motion_start.elapsed() >= player_motion_time {
            context.player.advance_all(&maze, context.direction);
            context.player_motion_start = Instant::now();
            motion_cycle = (motion_cycle + 1) & 3;
        }
        */
        if context.bullet_motion_start.elapsed() >= bullet_motion_time {
            // context.advance_bullets();
            context.bullet_motion_start = Instant::now();
        }
        if context.firing
            && context.bullet_fire_start.elapsed() >= bullet_firing_time
        {
            context.fire();
            context.bullet_fire_start = Instant::now();
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
        Keycode::S => {
            context.sticky_mode = !context.sticky_mode;
            if !context.sticky_mode {
                context.firing = false;
                context.stop(dir::NONE);
            }
        }
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
}

fn key_up(context: &mut GameContext, keycode: Keycode) {
    if !context.sticky_mode {
        match keycode {
            Keycode::Up => context.stop(dir::UP),
            Keycode::Down => context.stop(dir::DOWN),
            Keycode::Left => context.stop(dir::LEFT),
            Keycode::Right => context.stop(dir::RIGHT),
            Keycode::Space => context.firing = false,
            _ => {}
        }
    }
}
