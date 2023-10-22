use video::{ascii_font, init, Event, Keycode};

fn main() {
    let mut video = init(0).unwrap();
    let font = ascii_font().unwrap();

    enum Action {
        Nothing,
        ShowCharmap,
        FillScreen,
        Quit,
    }

    let mut action = Action::ShowCharmap;
    loop {
        match action {
            Action::Nothing => {}
            Action::ShowCharmap => {
                video.clear(0, 0, 0);
                _ = video.draw_font(&font);
                video.render();
            }
            Action::FillScreen => {
                video.clear(0, 0, 0);
                let mut ch = 0x20;
                for row in 0..33 {
                    for col in 0..80 {
                        _ = video.draw_char(row, col, ch, &font);
                        ch += 1;
                        if ch > 0x7f {
                            ch = 0x20
                        }
                    }
                }
                video.render();
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
