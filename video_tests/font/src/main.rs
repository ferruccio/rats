use clap::Parser;
use std::time::Instant;
use video::{init, Event, InitOptions, Keycode};

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
    _ = video.init_charmap();

    let mut running = true;
    let mut offset = 0;
    let mut start_ch: u8 = 0;
    let mut frames = 0;
    let start = Instant::now();
    while running {
        start_ch = start_ch.wrapping_add(offset as u8);
        let mut ch = start_ch;
        for row in 0..video.buffer.rows {
            for col in 0..video.buffer.cols {
                video.buffer.set(row, col, ch);
                ch = ch.wrapping_add(1);
            }
        }

        frames += 1;
        let seconds = start.elapsed().as_secs();
        let mut fps = frames / if seconds == 0 { 1 } else { seconds };
        let mut pos = 5;
        while fps != 0 {
            video.buffer.set(0, pos - 1, (fps % 10) as u8 + b'0');
            fps /= 10;
            pos -= 1;
        }
        while pos > 0 {
            video.buffer.set(0, pos - 1, b' ');
            pos -= 1;
        }

        _ = video.render_buffer();

        offset = 0;
        let cols = video.cols() as isize;
        video.handle_events(|event| match event {
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
                _ => {}
            },
            _ => {}
        });
    }
    println!(
        "rows: {rows}, cols: {cols}",
        rows = video.rows(),
        cols = video.cols()
    );
}
