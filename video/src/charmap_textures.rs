use crate::{
    buffer::{ATTR_DIM, ATTR_NONE, ATTR_REVERSE},
    charmaps::{ASCII, ASCII_START, MAZE_WALLS, MAZE_WALLS_START},
    errors::sdl_error,
    Result, Video, ATTR_COMBOS, BYTES_PER_PIXEL, CHAR_CELL_HEIGHT,
    CHAR_CELL_WIDTH, EMPTY, FONT_SIZE,
};
use sdl2::render::Texture;

impl Video {
    pub fn init_charmap_textures(
        &self,
        textures: &mut Vec<Texture>,
    ) -> Result<()> {
        assert!(textures.len() == ATTR_COMBOS);
        clear_charmap_textures(textures)?;
        load_charmap_textures(textures, &ASCII, ASCII_START)?;
        load_charmap_textures(textures, &MAZE_WALLS, MAZE_WALLS_START)?;
        Ok(())
    }
}

fn clear_charmap_textures(textures: &mut [Texture]) -> Result<()> {
    textures[ATTR_NONE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_empty(pixels);
        })
        .map_err(sdl_error)?;
    textures[ATTR_REVERSE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_empty(pixels);
        })
        .map_err(sdl_error)?;
    textures[ATTR_DIM as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_empty(pixels);
        })
        .map_err(sdl_error)?;
    textures[(ATTR_REVERSE | ATTR_DIM) as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_empty(pixels);
        })
        .map_err(sdl_error)?;
    Ok(())
}

fn load_charmap_textures(
    textures: &mut [Texture],
    bitmap: &[u8],
    first: u8,
) -> Result<()> {
    textures[ATTR_NONE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels(pixels, bitmap, first, ATTR_NONE);
        })
        .map_err(sdl_error)?;
    textures[ATTR_REVERSE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels(pixels, bitmap, first, ATTR_REVERSE);
        })
        .map_err(sdl_error)?;
    textures[ATTR_DIM as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels(pixels, bitmap, first, ATTR_DIM);
        })
        .map_err(sdl_error)?;
    textures[(ATTR_REVERSE | ATTR_DIM) as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels(pixels, bitmap, first, ATTR_REVERSE | ATTR_DIM);
        })
        .map_err(sdl_error)?;
    Ok(())
}

fn set_pixels(pixels: &mut [u8], bitmap: &[u8], first: u8, attrs: u8) {
    assert_eq!(
        pixels.len(),
        FONT_SIZE * BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT
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
            } else if byte & mask != 0 {
                intensity
            } else {
                0
            };
            pixels[offset + 2] = 0;
            offset += BYTES_PER_PIXEL;
            mask >>= 1;
        }
    }
}

fn set_pixels_empty(pixels: &mut [u8]) {
    assert_eq!(
        pixels.len(),
        FONT_SIZE * BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT
    );
    let mut offset = 0;
    for _ in 0..FONT_SIZE {
        for byte in EMPTY {
            let mut mask = 0x80;
            while mask != 0 {
                pixels[offset] = if byte & mask != 0 { 0x40 } else { 0 };
                pixels[offset + 1] = 0;
                pixels[offset + 2] = 0;
                offset += BYTES_PER_PIXEL;
                mask >>= 1;
            }
        }
    }
}
