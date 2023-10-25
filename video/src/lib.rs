use buffer::{Buffer, ATTR_COMBOS, ATTR_MASK};
use errors::sdl_error;
use sdl2::{
    pixels::Color, rect::Rect, render::Canvas, surface::Surface, video::Window,
    EventPump, Sdl, VideoSubsystem,
};

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;

mod buffer;
mod charmap_surface;
mod charmaps;
mod errors;
mod init;

pub use buffer::{ATTR_DIM, ATTR_NONE, ATTR_REVERSE};
pub use errors::Result;
pub use init::{init, InitOptions};

pub struct Video {
    _context: Sdl,
    _video: VideoSubsystem,
    bounds: Rect,
    scale: usize,
    rows: usize,
    cols: usize,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    charmap_surfaces: Vec<Surface<'static>>,
    pub buffer: Buffer,
    back_buffer: Buffer,
}

const CHARACTERS: usize = 256;
const BYTES_PER_PIXEL: usize = 3;
const CHAR_CELL_WIDTH: usize = 8;
const CHAR_CELL_HEIGHT: usize = 12;

impl Video {
    pub fn render(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self, red: u8, green: u8, blue: u8) {
        self.canvas.set_draw_color(Color::RGB(red, green, blue));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn height(&self) -> usize {
        self.bounds.height() as usize
    }

    pub fn width(&self) -> usize {
        self.bounds.width() as usize
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn handle_events<F>(&mut self, mut handler: F)
    where
        F: FnMut(Event),
    {
        for event in self.event_pump.poll_iter() {
            handler(event);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.buffer.swap(&mut self.back_buffer);
    }

    pub fn render_buffer(&mut self) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();
        let mut textures = vec![];
        for index in 0..ATTR_COMBOS {
            textures.push(
                self.charmap_surfaces[index].as_texture(&texture_creator)?,
            );
        }
        self.canvas
            .set_scale(self.scale as f32, self.scale as f32)
            .map_err(sdl_error)?;
        for row in 0..self.buffer.rows {
            for col in 0..self.buffer.cols {
                let ch = self.buffer.get_char(row, col);
                let attr = self.buffer.get_attr(row, col);
                let src = Rect::new(
                    0,
                    (ch as usize * CHAR_CELL_HEIGHT) as i32,
                    CHAR_CELL_WIDTH as u32,
                    CHAR_CELL_HEIGHT as u32,
                );
                let dst = Rect::new(
                    (col * CHAR_CELL_WIDTH) as i32,
                    (row * CHAR_CELL_HEIGHT) as i32,
                    CHAR_CELL_WIDTH as u32,
                    CHAR_CELL_HEIGHT as u32,
                );
                self.canvas
                    .copy(&textures[(attr & ATTR_MASK) as usize], src, dst)
                    .map_err(sdl_error)?;
            }
        }
        self.render();
        self.swap_buffers();
        self.buffer.clear();
        Ok(())
    }
}
