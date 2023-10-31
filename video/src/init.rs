use crate::{
    buffer::Buffer, errors::sdl_error, Pixels, Result, Video, CHAR_CELL_HEIGHT,
    CHAR_CELL_WIDTH,
};
use sdl2::rect::Rect;

#[derive(Debug, Default)]
pub struct InitOptions {
    display_index: Option<usize>,
    window_width: Option<usize>,
    window_height: Option<usize>,
    scale: usize,
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

    pub fn scale(mut self, scale: usize) -> Self {
        self.scale = scale;
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
    let scale = opts.scale.clamp(1, 4);
    let rows = (bounds.height() as usize / CHAR_CELL_HEIGHT / scale) as usize;
    let cols = (bounds.width() as usize / CHAR_CELL_WIDTH / scale) as usize;
    let window = video
        .window("", bounds.width(), bounds.height())
        .fullscreen()
        .position(bounds.x(), bounds.y())
        .build()?;
    sdl.mouse().show_cursor(false);
    let canvas = window.into_canvas().build()?;

    Ok(Video {
        sdl,
        bounds,
        scale,
        rows,
        cols,
        canvas,
        buffer: Buffer::new(rows, cols),
    })
}
