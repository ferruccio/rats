use std::time::Instant;

use video::{init, Event, Keycode};

fn main() {
    let mut video = init(0).unwrap();
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
            video.buffer.set(0, 74 + pos, (fps % 10) as u8 + b'0');
            fps /= 10;
            pos -= 1;
        }
        while pos > 0 {
            video.buffer.set(0, 74 + pos, b' ');
            pos -= 1;
        }

        _ = video.render_buffer();

        offset = 0;
        video.handle_events(|event| match event {
            Event::Quit { .. } => running = false,
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Escape | Keycode::Q => running = false,
                Keycode::Right => offset = 1,
                Keycode::Left => offset = -1,
                Keycode::Up => offset = -80,
                Keycode::Down => offset = 80,
                _ => {}
            },
            _ => {}
        });
    }
}
