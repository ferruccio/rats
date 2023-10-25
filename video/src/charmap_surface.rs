use crate::{
    buffer::{ATTR_DIM, ATTR_NONE, ATTR_REVERSE},
    charmaps::ASCII,
    errors::sdl_error,
    Result, Video, BYTES_PER_PIXEL, CHARACTERS, CHAR_CELL_HEIGHT,
    CHAR_CELL_WIDTH,
};
use sdl2::{pixels::Color, rect::Rect};

impl Video {
    // initialize all character maps to empty (blank with center red dot)
    pub fn init_charmap(&mut self) -> Result<()> {
        for surface in self.charmap_surfaces.iter_mut() {
            surface
                .fill_rect(
                    Rect::new(
                        0,
                        0,
                        CHAR_CELL_WIDTH as u32,
                        (CHARACTERS * CHAR_CELL_HEIGHT) as u32,
                    ),
                    Color::RGB(0, 0, 0),
                )
                .map_err(sdl_error)?;
            for ch in 0..256 {
                surface
                    .fill_rect(
                        Rect::new(
                            CHAR_CELL_WIDTH as i32 / 2 - 1,
                            ch * CHAR_CELL_HEIGHT as i32
                                + CHAR_CELL_WIDTH as i32 / 2
                                - 1,
                            2,
                            2,
                        ),
                        Color::RGB(255, 0, 0),
                    )
                    .map_err(sdl_error)?;
            }
        }
        self.load_charmap(&ASCII, 0x20);
        Ok(())
    }

    // create character map from bitmap
    pub fn load_charmap(&mut self, bitmap: &[u8], first: u8) {
        self.charmap_surfaces[ATTR_NONE as usize].with_lock_mut(|pixels| {
            set_pixels(pixels, bitmap, first, ATTR_NONE);
        });
        self.charmap_surfaces[ATTR_REVERSE as usize].with_lock_mut(|pixels| {
            set_pixels(pixels, bitmap, first, ATTR_REVERSE);
        });
        self.charmap_surfaces[ATTR_DIM as usize].with_lock_mut(|pixels| {
            set_pixels(pixels, bitmap, first, ATTR_DIM);
        });
        self.charmap_surfaces[(ATTR_REVERSE | ATTR_DIM) as usize]
            .with_lock_mut(|pixels| {
                set_pixels(pixels, bitmap, first, ATTR_REVERSE | ATTR_DIM);
            });
    }
}

fn set_pixels(pixels: &mut [u8], bitmap: &[u8], first: u8, attrs: u8) {
    assert_eq!(
        pixels.len(),
        CHARACTERS * BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT
    );
    let mut offset: usize =
        first as usize * BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT;
    let intensity = if attrs & ATTR_DIM == 0 { 0xff } else { 0x80 };
    for byte in bitmap {
        let mut mask = 0x80;
        while mask != 0 {
            pixels[offset] = 0;
            pixels[offset + 1] = if attrs & ATTR_REVERSE != 0 {
                if byte & mask == 0 {
                    intensity
                } else {
                    0
                }
            } else {
                if byte & mask != 0 {
                    intensity
                } else {
                    0
                }
            };
            pixels[offset + 2] = 0;
            offset += BYTES_PER_PIXEL;
            mask >>= 1;
        }
    }
}
