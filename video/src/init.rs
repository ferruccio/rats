use crate::{
    buffer::Buffer, errors::sdl_error, Pixels, Result, Size, SoundEffects,
    Video, CHAR_CELL_HEIGHT, CHAR_CELL_WIDTH,
};
use sdl2::{
    mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS},
    rect::Rect,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct InitOptions {
    pub display_index: Option<usize>,
    pub window_width: Option<usize>,
    pub window_height: Option<usize>,
    pub scale: Option<usize>,
    pub maze_height: Option<Size>,
    pub maze_width: Option<Size>,
    pub density: Option<usize>,
    pub factories: Option<usize>,
    pub quiet: bool,
}

impl InitOptions {
    pub fn new() -> InitOptions {
        InitOptions {
            ..InitOptions::default()
        }
    }

    pub fn display_index(mut self, index: Option<usize>) -> Self {
        if let Some(index) = index {
            self.display_index = Some(index);
        }
        self
    }

    pub fn window_width(mut self, width: Option<Pixels>) -> Self {
        if let Some(width) = width {
            if width >= 640 {
                // make sure width is a multiple of CHAR_CELL_WIDTH
                self.window_width = Some(width - width % CHAR_CELL_WIDTH);
            }
        }
        self
    }

    pub fn window_height(mut self, height: Option<Pixels>) -> Self {
        if let Some(height) = height {
            if height >= 480 {
                // make sure height is a multiple of CHAR_CELL_HEIGHT
                self.window_height = Some(height - height % CHAR_CELL_HEIGHT);
            }
        }
        self
    }

    pub fn scale(mut self, scale: Option<usize>) -> Self {
        self.scale = scale;
        self
    }

    pub fn maze_height(mut self, maze_height: Option<Size>) -> Self {
        self.maze_height = maze_height;
        self
    }

    pub fn maze_width(mut self, maze_width: Option<Size>) -> Self {
        self.maze_width = maze_width;
        self
    }

    pub fn density(mut self, density: Option<usize>) -> Self {
        self.density = density;
        self
    }

    pub fn factories(mut self, factories: Option<usize>) -> Self {
        self.factories = factories;
        self
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }
}

pub fn init(opts: InitOptions) -> Result<Video> {
    let sdl = sdl2::init().map_err(sdl_error)?;
    let video = sdl.video().map_err(sdl_error)?;
    let bounds = video
        .display_bounds(opts.display_index.unwrap_or(0) as i32)
        .map_err(sdl_error)?;
    let bounds = Rect::new(
        bounds.x(),
        bounds.y(),
        opts.window_width.unwrap_or(bounds.width() as usize) as u32,
        opts.window_height.unwrap_or(bounds.height() as usize) as u32,
    );
    let scale =
        opts.scale
            .map(|scale| scale.clamp(1, 4))
            .unwrap_or_else(|| {
                for scale in 1..=4 {
                    if bounds.width() / scale / 8 <= 80 {
                        return scale as usize;
                    }
                }
                4
            });
    let rows = (bounds.height() as usize / CHAR_CELL_HEIGHT / scale) as Size;
    let cols = (bounds.width() as usize / CHAR_CELL_WIDTH / scale) as Size;
    let window = video
        .window("", bounds.width(), bounds.height())
        .fullscreen()
        .position(bounds.x(), bounds.y())
        .build()?;
    sdl.mouse().show_cursor(false);
    let canvas = window.into_canvas().build()?;

    sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)
        .map_err(sdl_error)?;
    sdl2::mixer::allocate_channels(20);

    Ok(Video {
        sdl,
        bounds,
        scale,
        rows,
        cols,
        canvas,
        sounds: SoundEffects::new()?,
        buffer: Buffer::new(rows, cols),
        quiet: opts.quiet,
    })
}
