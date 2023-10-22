use std::{thread::sleep, time::Duration};

use video::init;

fn main() {
    match init(0) {
        Ok(mut video) => {
            for i in 0..255 {
                // cycle from black to magenta
                video.clear(i, 0, i);
                sleep(Duration::from_millis(10));
            }
        }
        Err(error) => {
            println!("{error}");
        }
    }
}
