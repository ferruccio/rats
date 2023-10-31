use crate::{
    buffer::{ATTR_DIM, ATTR_NONE, ATTR_REVERSE},
    charmaps::{ASCII, ASCII_START, MAZE_WALLS, MAZE_WALLS_START},
    errors::sdl_error,
    Result, Video, ATTR_COMBOS, BYTES_PER_PIXEL, CHAR_CELL_HEIGHT,
    CHAR_CELL_WIDTH, EMPTY_CHAR_CELL, FONT_SIZE, PLAYER, PLAYER_START,
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
        load_charmap_textures_wide(textures, &PLAYER, PLAYER_START)?;
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
    assert_eq!(bitmap.len() % CHAR_CELL_HEIGHT, 0);
    assert_eq!(
        pixels.len(),
        FONT_SIZE * BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT
    );
    let mut offset: usize =
        first as usize * BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT;
    const BACKGROUND: u32 = 0x252919;
    const FOREGROUND_NORMAL: u32 = 0x5bffff;
    const FOREGROUND_DIM: u32 = 0x2d8080;
    let (fg, bg) = match (attrs & ATTR_REVERSE != 0, attrs & ATTR_DIM != 0) {
        (true, true) => (BACKGROUND, FOREGROUND_DIM),
        (true, false) => (BACKGROUND, FOREGROUND_NORMAL),
        (false, true) => (FOREGROUND_DIM, BACKGROUND),
        (false, false) => (FOREGROUND_NORMAL, BACKGROUND),
    };
    for byte in bitmap {
        let mut mask = 0x80;
        while mask != 0 {
            if byte & mask != 0 {
                pixels[offset] = ((fg >> 16) & 0xff) as u8;
                pixels[offset + 1] = ((fg >> 8) & 0xff) as u8;
                pixels[offset + 2] = (fg & 0xff) as u8;
            } else {
                pixels[offset] = ((bg >> 16) & 0xff) as u8;
                pixels[offset + 1] = ((bg >> 8) & 0xff) as u8;
                pixels[offset + 2] = (bg & 0xff) as u8;
            }
            offset += BYTES_PER_PIXEL;
            mask >>= 1;
        }
    }
}

fn load_charmap_textures_wide(
    textures: &mut [Texture],
    bitmap: &[u16],
    first: u8,
) -> Result<()> {
    textures[ATTR_NONE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_wide(pixels, bitmap, first, ATTR_NONE);
        })
        .map_err(sdl_error)?;
    textures[ATTR_REVERSE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_wide(pixels, bitmap, first, ATTR_REVERSE);
        })
        .map_err(sdl_error)?;
    textures[ATTR_DIM as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_wide(pixels, bitmap, first, ATTR_DIM);
        })
        .map_err(sdl_error)?;
    textures[(ATTR_REVERSE | ATTR_DIM) as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_wide(pixels, bitmap, first, ATTR_REVERSE | ATTR_DIM);
        })
        .map_err(sdl_error)?;
    Ok(())
}

fn set_pixels_wide(
    pixels: &mut [u8],
    mut bitmap: &[u16],
    mut first: u8,
    attrs: u8,
) {
    assert_eq!(bitmap.len() % (CHAR_CELL_HEIGHT * 2), 0);
    while bitmap.len() > 0 {
        let mut bitmap2: [u8; CHAR_CELL_HEIGHT * 4] = [0; CHAR_CELL_HEIGHT * 4];
        let mut offset = 0;
        for i in 0..CHAR_CELL_HEIGHT {
            bitmap2[offset] = (bitmap[i] >> 8) as u8;
            offset += 1;
        }
        for i in 0..CHAR_CELL_HEIGHT {
            bitmap2[offset] = (bitmap[i] & 0xff) as u8;
            offset += 1;
        }
        for i in CHAR_CELL_HEIGHT..CHAR_CELL_HEIGHT * 2 {
            bitmap2[offset] = (bitmap[i] >> 8) as u8;
            offset += 1;
        }
        for i in CHAR_CELL_HEIGHT..CHAR_CELL_HEIGHT * 2 {
            bitmap2[offset] = (bitmap[i] & 0xff) as u8;
            offset += 1;
        }
        set_pixels(pixels, &bitmap2, first, attrs);
        bitmap = &bitmap[CHAR_CELL_HEIGHT * 2..];
        first += 4;
    }
}

fn set_pixels_empty(pixels: &mut [u8]) {
    assert_eq!(
        pixels.len(),
        FONT_SIZE * BYTES_PER_PIXEL * CHAR_CELL_WIDTH * CHAR_CELL_HEIGHT
    );
    let mut offset = 0;
    for _ in 0..FONT_SIZE {
        for byte in EMPTY_CHAR_CELL {
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
