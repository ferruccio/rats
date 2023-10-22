use errors::{sdl_error, Result};
use sdl2::{
    pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl,
    VideoSubsystem,
};

mod ascii;
mod errors;

pub struct Video {
    _context: Sdl,
    _video: VideoSubsystem,
    bounds: Rect,
    canvas: Canvas<Window>,
}

pub fn init(display_index: i32) -> Result<Video> {
    let context = sdl2::init().map_err(sdl_error)?;
    let video = context.video().map_err(sdl_error)?;
    let bounds = video.display_bounds(display_index).map_err(sdl_error)?;
    let window = video
        .window("", bounds.width(), bounds.height())
        .fullscreen()
        .position_centered()
        .build()?;
    let canvas = window.into_canvas().build()?;

    Ok(Video {
        _context: context,
        _video: video,
        bounds,
        canvas,
    })
}

impl Video {
    pub fn clear(&mut self, red: u8, green: u8, blue: u8) {
        self.canvas.set_draw_color(Color::RGB(red, green, blue));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn height(&self) -> u32 {
        self.bounds.height()
    }

    pub fn width(&self) -> u32 {
        self.bounds.width()
    }
}
