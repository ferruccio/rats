use clap::Parser;
use std::time::Instant;
use video::{
    init, Event, InitOptions, Keycode, PixelFormatEnum, ATTR_COMBOS, ATTR_DIM,
    ATTR_NONE, ATTR_REVERSE, CHAR_CELL_HEIGHT, CHAR_CELL_WIDTH, FONT_SIZE,
};

#[derive(Parser, Debug)]
struct Options {
    /// Display index
    #[clap(short = 'd', long = "display")]
    display: Option<usize>,

    /// Window width
    #[clap(short = 'x', long = "x-width")]
    width: Option<usize>,

    /// Window height
    #[clap(short = 'y', long = "y-height")]
    height: Option<usize>,

    /// Scale factor (1 to 4)
    #[clap(short = 's', long = "scale")]
    scale: Option<usize>,
}

fn main() {
    let opts = Options::parse();
    let mut video = init(
        InitOptions::new()
            .display_index(opts.display)
            .window_height(opts.height)
            .window_width(opts.width)
            .scale(opts.scale),
    )
    .unwrap();
    let texture_creator = video.canvas.texture_creator();
    let mut textures = vec![];
    for _ in 0..ATTR_COMBOS {
        let texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGB24,
                (CHAR_CELL_WIDTH * video.scale) as u32,
                (FONT_SIZE as usize * CHAR_CELL_HEIGHT * video.scale) as u32,
            )
            .unwrap();
        textures.push(texture);
    }
    _ = video.init_charmap_textures(&mut textures, video.scale);
    let mut event_pump = video.sdl.event_pump().unwrap();

    let mut running = true;
    let mut offset = 0;
    let mut start_ch: u8 = 0;
    let mut reverse = false;
    let mut dim = false;
    let mut frames = 0;
    let start = Instant::now();
    while running {
        start_ch = start_ch.wrapping_add(offset as u8);
        let mut ch = start_ch;
        for row in 0..video.buffer.rows {
            for col in 0..video.buffer.cols {
                video.buffer.set_char(row, col, ch);
                let mut attr = ATTR_NONE;
                if reverse {
                    attr |= ATTR_REVERSE;
                }
                if dim {
                    attr |= ATTR_DIM;
                }
                video.buffer.set_attr(row, col, attr);
                ch = ch.wrapping_add(1);
            }
        }

        frames += 1;
        let seconds = start.elapsed().as_secs();
        video.buffer.print(
            0,
            0,
            ATTR_REVERSE | ATTR_DIM,
            format!(
                "FPS: {fps}",
                fps = frames / if seconds == 0 { 1 } else { seconds }
            ),
        );

        _ = video.render_buffer(&textures);

        offset = 0;
        let cols = video.cols() as isize;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape | Keycode::Q => running = false,
                    Keycode::Right => offset = 1,
                    Keycode::Left => offset = -1,
                    Keycode::Up => offset = -cols,
                    Keycode::Down => offset = cols,
                    Keycode::R => reverse = !reverse,
                    Keycode::D => dim = !dim,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    println!(
        "rows: {rows}, cols: {cols}",
        rows = video.rows(),
        cols = video.cols()
    );
}
