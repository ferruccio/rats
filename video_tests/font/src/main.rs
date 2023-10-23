use video::{init, Event, Keycode};

fn main() {
    let mut video = init(0).unwrap();
    _ = video.init_charmap();

    enum Action {
        Nothing,
        ShowCharmap,
        FillScreen,
        Quit,
    }

    let mut action = Action::ShowCharmap;
    let mut start_ch: u8 = 0x20;
    loop {
        match action {
            Action::Nothing => {}
            Action::ShowCharmap => {
                _ = video.draw_font();
            }
            Action::FillScreen => {
                let mut ch = start_ch;
                start_ch += 1;
                if start_ch > 0x7f {
                    start_ch = 0x20;
                }
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
            Action::Quit => {
                break;
            }
        }
        action = Action::Nothing;
        video.handle_events(|event| match event {
            Event::Quit { .. } => {
                action = Action::Quit;
            }
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::Escape | Keycode::Q => {
                    action = Action::Quit;
                }
                Keycode::Space | Keycode::F => {
                    action = Action::FillScreen;
                }
                Keycode::C => action = Action::ShowCharmap,
                _ => {}
            },
            _ => {}
        });
    }
}
