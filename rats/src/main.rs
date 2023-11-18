use crate::game_context::random;
use clap::Parser;
use config::{
    BRAT_KILL, BRAT_SPAWN_SECONDS, RATS_PER_FACTORY, RAT_SPAWN_SECONDS,
};
use entities::dir;
use game_context::{GameContext, GameState};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use video::{
    sdl_error, Event, InitOptions, Keycode, PixelFormatEnum, Pixels, Result,
    Size, ATTR_COMBOS, CHAR_CELL_HEIGHT, CHAR_CELL_WIDTH, FONT_SIZE,
};

mod config;
mod entities;
mod game_context;
mod maze;

#[derive(Parser)]
struct CommandLineOpts {
    /// Display index
    #[clap(short = 'd', long = "display")]
    display: Option<usize>,

    /// Classic mode
    #[clap(short = 'c', long = "classic", action)]
    classic: bool,

    /// Number of rat factories
    #[clap(short = 'f', long = "rat-factories")]
    factories: Option<usize>,

    /// Maze height (maze cells)
    #[clap(short = 'H', long = "maze-height", alias = "mh")]
    maze_height: Option<Size>,

    /// Maze width (maze cells)
    #[clap(short = 'W', long = "maze-width", alias = "mw")]
    maze_width: Option<Size>,

    /// Maze density
    #[clap(short = 'm', long = "maze-density")]
    density: Option<usize>,

    /// Window height (pixels)
    #[clap(long = "window-height", alias = "wh")]
    window_height: Option<Pixels>,

    /// Window width (pixels)
    #[clap(long = "window-width", alias = "ww")]
    window_width: Option<Pixels>,

    /// Scale factor (1 to 4)
    #[clap(short = 's', long = "scale")]
    scale: Option<usize>,

    /// Limit FPS (0 = no limit)
    #[clap(long = "fps", default_value_t = 60, hide = true)]
    fps: usize,

    /// Rat damage (decrease health)
    #[clap(
        short = 'r',
        long = "rat-damage",
        default_value_t = 50,
        conflicts_with = "classic"
    )]
    rat_damage: usize,

    /// Brat damage (decrease health)
    #[clap(
        short = 'b',
        long = "brat-damage",
        default_value_t = 25,
        conflicts_with = "classic"
    )]
    brat_damage: usize,
}

fn main() {
    let mut opts = CommandLineOpts::parse();
    if opts.classic {
        opts.rat_damage = 100;
        opts.brat_damage = 100;
    }
    if let Err(error) = play(opts) {
        println!("{error}");
    }
}

fn play(opts: CommandLineOpts) -> Result<()> {
    let mut context = GameContext::create(
        InitOptions::new()
            .display_index(opts.display)
            .window_height(opts.window_height)
            .window_width(opts.window_width)
            .scale(opts.scale)
            .maze_height(opts.maze_height)
            .maze_width(opts.maze_width)
            .density(opts.density)
            .factories(opts.factories),
        opts.rat_damage,
        opts.brat_damage,
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
    context.video.init_charmap_textures(
        &mut textures,
        context.video.scale,
        !opts.classic,
    )?;

    let nanos_per_frame = if opts.fps > 0 {
        (1_000_000_000 / opts.fps) as u32
    } else {
        0
    };
    let mut frame_time = Instant::now();
    let mut event_pump = context.video.sdl.event_pump().map_err(sdl_error)?;
    let mut rat_spawn_time =
        Instant::now() - Duration::new(RAT_SPAWN_SECONDS, 0);
    let mut brat_spawn_time = Instant::now();
    while context.game_state != GameState::Quit {
        context.render_frame(&textures, opts.classic)?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => context.game_state = GameState::Quit,
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
        if context.game_state == GameState::Restart {
            context.new_game(
                InitOptions::new()
                    .display_index(opts.display)
                    .window_height(opts.window_height)
                    .window_width(opts.window_width)
                    .scale(opts.scale)
                    .maze_height(opts.maze_height)
                    .maze_width(opts.maze_width)
                    .density(opts.density)
                    .factories(opts.factories),
            );
        }

        context.update();
        if context.firing_dir != dir::NONE
            && context.bullet_fire_start.elapsed() >= context.bullet_firing_time
        {
            context.fire();
        }
        if rat_spawn_time.elapsed().as_secs() >= RAT_SPAWN_SECONDS {
            context.new_rats =
                (context.live_factories as f32 * RATS_PER_FACTORY) as usize;
            rat_spawn_time = Instant::now();
        }
        if context.live_rats > 0
            && brat_spawn_time.elapsed().as_secs() >= BRAT_SPAWN_SECONDS
        {
            // Breed you little bastards!
            context.new_brats = context.live_rats / 8 + random(2, 10);
            brat_spawn_time = Instant::now();
        }
        if context.game_state != GameState::Quit
            && ((context.live_factories == 0 && context.live_rats == 0)
                || context.players_left == 0)
        {
            if !opts.classic {
                context.score += context.live_brats * BRAT_KILL;
                context.dead_brats += context.live_brats;
                context.live_brats = 0;
            }
            if context.live_brats == 0 {
                if context.game_state != GameState::Finished {
                    context.time = context.start.elapsed().as_secs() as usize;
                }
                context.game_state = GameState::Finished;
            }
        }

        if nanos_per_frame > 0 {
            // don't hog the CPU
            let nanos_elapsed = frame_time.elapsed().as_nanos() as u32;
            if nanos_elapsed < nanos_per_frame {
                sleep(Duration::new(0, nanos_per_frame - nanos_elapsed));
            }
        }

        frame_time = Instant::now();
    }

    Ok(())
}

// return true to keep game running
fn key_down(context: &mut GameContext, keycode: Keycode) {
    match keycode {
        Keycode::Escape => context.game_state = GameState::Quit,
        Keycode::Space => match context.game_state {
            GameState::Running => context.game_state = GameState::Paused,
            GameState::Paused => context.game_state = GameState::Running,
            _ => {}
        },
        Keycode::F12 => context.diagnostics = !context.diagnostics,
        Keycode::Up => context.start(dir::UP),
        Keycode::Down => context.start(dir::DOWN),
        Keycode::Left => context.start(dir::LEFT),
        Keycode::Right => context.start(dir::RIGHT),
        Keycode::W => context.start_firing(dir::UP),
        Keycode::A => context.start_firing(dir::LEFT),
        Keycode::S => context.start_firing(dir::DOWN),
        Keycode::D => context.start_firing(dir::RIGHT),
        Keycode::Y if context.game_state == GameState::Finished => {
            context.game_state = GameState::Restart;
        }
        Keycode::N if context.game_state == GameState::Finished => {
            context.game_state = GameState::Quit;
        }
        _ => {}
    }
}

fn key_up(context: &mut GameContext, keycode: Keycode) {
    match keycode {
        Keycode::Up => context.stop(dir::UP),
        Keycode::Down => context.stop(dir::DOWN),
        Keycode::Left => context.stop(dir::LEFT),
        Keycode::Right => context.stop(dir::RIGHT),
        Keycode::W => context.stop_firing(dir::UP),
        Keycode::A => context.stop_firing(dir::LEFT),
        Keycode::S => context.stop_firing(dir::DOWN),
        Keycode::D => context.stop_firing(dir::RIGHT),
        _ => {}
    }
}
