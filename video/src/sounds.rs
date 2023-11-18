use crate::{sdl_error, Result, Video};
use sdl2::mixer::{Channel, Chunk};

impl Video {
    pub fn play_gunshot(&self) {
        self.play(&self.sounds.gunshot);
    }

    pub fn play_impact(&self) {
        self.play(&self.sounds.impact);
    }

    pub fn play_short_explosion(&self) {
        self.play(&self.sounds.short_explosion);
    }

    pub fn play_long_explosion(&self) {
        self.play(&self.sounds.long_explosion);
    }

    fn play(&self, chunk: &Chunk) {
        if !self.quiet {
            if let Err(err) = Channel::all().play(chunk, 0) {
                println!("sound error: {err}");
            }
        }
    }
}

pub struct SoundEffects {
    gunshot: Chunk,
    impact: Chunk,
    short_explosion: Chunk,
    long_explosion: Chunk,
}

impl SoundEffects {
    pub fn new() -> Result<SoundEffects> {
        let gunshot_audio =
            Vec::from(include_bytes!("../sounds/Retro Gun SingleShot 04.wav"));
        let impact_audio = Vec::from(include_bytes!(
            "../sounds/Retro Impact Punch Hurt 01.wav"
        ));
        let short_explosion_audio =
            Vec::from(include_bytes!("../sounds/Retro Explosion Short 15.wav"));
        let long_explosion_audio =
            Vec::from(include_bytes!("../sounds/Retro Explosion Long 02.wav"));

        Ok(SoundEffects {
            gunshot: Chunk::from_raw_buffer(Box::from(gunshot_audio))
                .map_err(sdl_error)?,
            impact: Chunk::from_raw_buffer(Box::from(impact_audio))
                .map_err(sdl_error)?,
            short_explosion: Chunk::from_raw_buffer(Box::from(
                short_explosion_audio,
            ))
            .map_err(sdl_error)?,
            long_explosion: Chunk::from_raw_buffer(Box::from(
                long_explosion_audio,
            ))
            .map_err(sdl_error)?,
        })
    }
}
