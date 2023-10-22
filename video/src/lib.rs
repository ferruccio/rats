use charmaps::ASCII;
use errors::{sdl_error, Result};
use font::Font;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::Canvas,
    surface::Surface,
    video::Window,
    EventPump, Sdl, VideoSubsystem,
};

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;

mod charmaps;
mod errors;
mod font;

pub struct Video {
    _context: Sdl,
    _video: VideoSubsystem,
    bounds: Rect,
    canvas: Canvas<Window>,
    event_pump: EventPump,
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
    let event_pump = context.event_pump().map_err(sdl_error)?;

    Ok(Video {
        _context: context,
        _video: video,
        bounds,
        canvas,
        event_pump,
    })
}

const CHARACTERS: u32 = 256;
const BYTES_PER_PIXEL: u32 = 3;
const CHAR_CELL_WIDTH: u32 = 8;
const CHAR_CELL_HEIGHT: u32 = 12;

impl Video {
    pub fn render(&mut self) {
        self.canvas.present();
    }

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

    pub fn handle_events<F>(&mut self, mut handler: F)
    where
        F: FnMut(Event) -> (),
    {
        for event in self.event_pump.poll_iter() {
            handler(event);
        }
    }

    pub fn draw_char(
        &mut self,
        row: usize,
        col: usize,
        ch: u8,
        font: &Font,
    ) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();
        let texture = font.surface.as_texture(&texture_creator)?;
        self.canvas.set_scale(3.0, 3.0).map_err(sdl_error)?;
        _ = self.canvas.copy(
            &texture,
            Rect::new(
                0,
                ch as i32 * CHAR_CELL_HEIGHT as i32,
                CHAR_CELL_WIDTH,
                CHAR_CELL_HEIGHT,
            ),
            Rect::new(
                col as i32 * CHAR_CELL_WIDTH as i32,
                row as i32 * CHAR_CELL_HEIGHT as i32,
                CHAR_CELL_WIDTH,
                CHAR_CELL_HEIGHT,
            ),
        );
        Ok(())
    }

    // this is for diagnostic use only
    pub fn draw_font(&mut self, font: &Font) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();
        let texture = font.surface.as_texture(&texture_creator)?;
        self.canvas.set_scale(4.0, 4.0).map_err(sdl_error)?;
        for col in 0..16 {
            let src = Rect::new(
                0,
                col * 16 * CHAR_CELL_HEIGHT as i32,
                CHAR_CELL_WIDTH,
                CHAR_CELL_HEIGHT * 16,
            );
            let dst =
                Rect::new(col * 16, 0, CHAR_CELL_WIDTH, CHAR_CELL_HEIGHT * 16);
            _ = self.canvas.copy(&texture, src, dst);
        }
        Ok(())
    }
}

pub fn ascii_font() -> Result<Font<'static>> {
    create_font(&ASCII, 0x20)
}

fn create_font(font: &[u8], first: u8) -> Result<Font> {
    let mut surface = Surface::new(
        CHAR_CELL_WIDTH,
        256 * CHAR_CELL_HEIGHT,
        PixelFormatEnum::RGB24,
    )
    .map_err(sdl_error)?;
    // paint it black
    surface
        .fill_rect(
            Rect::new(0, 0, CHAR_CELL_WIDTH, CHARACTERS * CHAR_CELL_HEIGHT),
            Color::RGB(0, 50, 0),
        )
        .map_err(sdl_error)?;
    surface.with_lock_mut(|pixels| {
        assert_eq!(
            pixels.len(),
            (CHARACTERS * BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT)
                as usize
        );
        let mut offset: usize = first as usize
            * (BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT) as usize;
        for byte in font {
            let mut mask = 0x80;
            while mask != 0 {
                // set pixel color only if bit is 1
                if byte & mask != 0 {
                    // set the G in RGB
                    pixels[offset + 1] = 0xff;
                }
                offset += BYTES_PER_PIXEL as usize;
                mask >>= 1;
            }
        }
    });
    Ok(Font::new(surface))
}
