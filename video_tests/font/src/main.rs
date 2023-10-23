use video::{init, Event, Keycode};

fn main() {
    let mut video = init(0).unwrap();
    _ = video.init_charmap();

    enum Action {
        Nothing,
        ShowCharmap,
        FillScreen(i32),
        Quit,
    }

    let mut action = Action::FillScreen(0);
    let mut start_ch: i32 = 0x20;
    loop {
        match action {
            Action::Nothing => {}
            Action::ShowCharmap => {
                _ = video.draw_font();
            }
            Action::FillScreen(offset) => {
                start_ch += offset;
                if start_ch < 0x20 {
                    start_ch = 0x7f - (0x20 - start_ch) + 1;
                } else if start_ch > 0x7f {
                    start_ch = 0x20 + (start_ch - 0x7f) - 1;
                }
                let mut ch = start_ch as u8;
                for row in 0..video.buffer.rows {
                    for col in 0..video.buffer.cols {
                        video.buffer.set(row, col, ch);
                        ch += 1;
                        if ch > 0x7f {
                            ch = 0x20
                        }
                    }
                }
                _ = video.render_buffer();
            }
            Action::Quit => break,
        }
        action = Action::Nothing;
        video.handle_events(|event| match event {
            Event::Quit { .. } => action = Action::Quit,
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Escape | Keycode::Q => action = Action::Quit,
                Keycode::Space => action = Action::ShowCharmap,
                Keycode::Right => action = Action::FillScreen(1),
                Keycode::Left => action = Action::FillScreen(-1),
                Keycode::Up => action = Action::FillScreen(-80),
                Keycode::Down => action = Action::FillScreen(80),
                _ => {}
            },
            _ => {}
        });
    }
}
