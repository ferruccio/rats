use crate::{
    buffer::{Buffer, ATTR_COMBOS},
    errors::sdl_error,
    Result, Video, CHARACTERS, CHAR_CELL_HEIGHT, CHAR_CELL_WIDTH,
};
use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface};

#[derive(Debug, Default)]
pub struct InitOptions {
    display_index: Option<usize>,
    window_width: Option<usize>,
    window_height: Option<usize>,
    scale: Option<usize>,
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

    pub fn window_width(mut self, width: Option<usize>) -> Self {
        if let Some(width) = width {
            if width >= 640 {
                // make sure width is a multiple of CHAR_CELL_WIDTH
                self.window_width = Some(width - width % CHAR_CELL_WIDTH);
            }
        }
        self
    }

    pub fn window_height(mut self, height: Option<usize>) -> Self {
        if let Some(height) = height {
            if height >= 480 {
                // make sure height is a multiple of CHAR_CELL_HEIGHT
                self.window_height = Some(height - height % CHAR_CELL_HEIGHT);
            }
        }
        self
    }

    pub fn scale(mut self, scale: Option<usize>) -> Self {
        if let Some(scale) = scale {
            if scale > 0 && scale < 5 {
                self.scale = Some(scale);
            }
        }
        self
    }
}

pub fn init(opts: InitOptions) -> Result<Video> {
    let context = sdl2::init().map_err(sdl_error)?;
    let video = context.video().map_err(sdl_error)?;
    let bounds = video
        .display_bounds(opts.display_index.unwrap_or(0) as i32)
        .map_err(sdl_error)?;
    let bounds = Rect::new(
        bounds.x(),
        bounds.y(),
        opts.window_width.unwrap_or(bounds.width() as usize) as u32,
        opts.window_height.unwrap_or(bounds.height() as usize) as u32,
    );
    let scale = opts.scale.unwrap_or(1);
    let rows = (bounds.height() as usize / CHAR_CELL_HEIGHT / scale) as usize;
    let cols = (bounds.width() as usize / CHAR_CELL_WIDTH / scale) as usize;
    let window = video
        .window("", bounds.width(), bounds.height())
        .fullscreen()
        .position(bounds.x(), bounds.y())
        .build()?;
    let canvas = window.into_canvas().build()?;
    let event_pump = context.event_pump().map_err(sdl_error)?;

    let mut charmap_surfaces = vec![];
    for _ in 0..ATTR_COMBOS {
        charmap_surfaces.push(
            Surface::new(
                CHAR_CELL_WIDTH as u32,
                (CHARACTERS * CHAR_CELL_HEIGHT) as u32,
                PixelFormatEnum::RGB24,
            )
            .map_err(sdl_error)?,
        );
    }

    Ok(Video {
        _context: context,
        _video: video,
        bounds,
        scale,
        rows,
        cols,
        canvas,
        event_pump,
        charmap_surfaces,
        buffer: Buffer::new(rows, cols),
        back_buffer: Buffer::new(rows, cols),
    })
}
