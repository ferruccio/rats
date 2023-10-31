use buffer::ATTR_MASK;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
    Sdl,
};

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;

mod buffer;
mod charmap_textures;
mod charmaps;
mod errors;
mod init;

pub use buffer::{Buffer, ATTR_COMBOS, ATTR_DIM, ATTR_NONE, ATTR_REVERSE};
pub use charmaps::*;
pub use errors::{sdl_error, Result};
pub use init::{init, InitOptions};
pub use sdl2::pixels::PixelFormatEnum;

// use Pixels for bitmap dimensions
pub type Pixels = usize;
// use Chars for character map dimensions & positions
pub type Chars = usize;

pub struct Video {
    pub sdl: Sdl,
    bounds: Rect,
    scale: usize,
    rows: Chars,
    cols: Chars,
    pub canvas: Canvas<Window>,
    pub buffer: Buffer,
}

pub const FONT_SIZE: Chars = 256;
pub const BYTES_PER_PIXEL: usize = 3;
pub const CHAR_CELL_WIDTH: Pixels = 8;
pub const CHAR_CELL_HEIGHT: Pixels = 12;

impl Video {
    pub fn render(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self, red: u8, green: u8, blue: u8) {
        self.canvas.set_draw_color(Color::RGB(red, green, blue));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn height(&self) -> Pixels {
        self.bounds.height() as Pixels
    }

    pub fn width(&self) -> Pixels {
        self.bounds.width() as Pixels
    }

    pub fn rows(&self) -> Chars {
        self.rows
    }

    pub fn cols(&self) -> Chars {
        self.cols
    }

    pub fn render_buffer(&mut self, textures: &[Texture]) -> Result<()> {
        for row in 0..self.buffer.rows {
            for col in 0..self.buffer.cols {
                let ch = self.buffer.get_char(row, col);
                let attr = self.buffer.get_attr(row, col);
                let src = Rect::new(
                    0,
                    (ch as usize * CHAR_CELL_HEIGHT * self.scale) as i32,
                    (CHAR_CELL_WIDTH * self.scale) as u32,
                    (CHAR_CELL_HEIGHT * self.scale) as u32,
                );
                let dst = Rect::new(
                    (col * CHAR_CELL_WIDTH * self.scale) as i32,
                    (row * CHAR_CELL_HEIGHT * self.scale) as i32,
                    (CHAR_CELL_WIDTH * self.scale) as u32,
                    (CHAR_CELL_HEIGHT * self.scale) as u32,
                );
                self.canvas
                    .copy(&textures[(attr & ATTR_MASK) as usize], src, dst)
                    .map_err(sdl_error)?;
            }
        }
        self.render();
        Ok(())
    }
}

pub trait Wrapping {
    fn inc(self, count: usize) -> Self;
    fn dec(self, count: usize) -> Self;
}

impl Wrapping for usize {
    fn inc(self, count: usize) -> Self {
        if self < count - 1 {
            self + 1
        } else {
            0
        }
    }

    fn dec(self, count: usize) -> Self {
        if self > 0 {
            self - 1
        } else {
            count - 1
        }
    }
}
