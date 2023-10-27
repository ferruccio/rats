use buffer::{ATTR_COMBOS, ATTR_MASK};
use sdl2::{
    pixels::Color, rect::Rect, render::Canvas, surface::Surface, video::Window,
    Sdl,
};

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;

mod buffer;
mod charmap_surface;
mod charmaps;
mod errors;
mod init;

pub use buffer::{Buffer, ATTR_DIM, ATTR_NONE, ATTR_REVERSE};
pub use charmaps::{
    MAZE_ACROSS, MAZE_BOTTOM_LEFT, MAZE_BOTTOM_RIGHT, MAZE_BOTTOM_T,
    MAZE_CROSS, MAZE_DOWN, MAZE_LEFT_T, MAZE_RIGHT_T, MAZE_TOP_LEFT,
    MAZE_TOP_RIGHT, MAZE_TOP_T,
};
pub use errors::{sdl_error, Result};
pub use init::{init, InitOptions};

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
    canvas: Canvas<Window>,
    charmap_surfaces: Vec<Surface<'static>>,
    pub buffer: Buffer,
    back_buffer: Buffer,
}

const FONT_SIZE: Chars = 256;
const BYTES_PER_PIXEL: usize = 3;
const CHAR_CELL_WIDTH: Pixels = 8;
const CHAR_CELL_HEIGHT: Pixels = 12;

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
