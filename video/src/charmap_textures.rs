use crate::{
    buffer::{ATTR_DIM, ATTR_NONE, ATTR_REVERSE},
    charmaps::{ASCII, ASCII_START, MAZE_WALLS, MAZE_WALLS_START},
    errors::sdl_error,
    Result, Video, ATTR_COMBOS, BRATS, BRATS_START, BULLETS, BULLETS_START,
    BYTES_PER_PIXEL, CHAR_CELL_HEIGHT, CHAR_CELL_WIDTH, EMPTY_CHAR_CELL,
    FACTORIES, FACTORIES_START, FONT_SIZE, PLAYER, PLAYER_START, RATS,
    RATS_START,
};
use sdl2::render::Texture;

impl Video {
    pub fn init_charmap_textures(
        &self,
        textures: &mut Vec<Texture>,
        scale: usize,
    ) -> Result<()> {
        assert!(textures.len() == ATTR_COMBOS);
        clear_charmap_textures(textures, scale)?;
        charmap_textures(textures, scale, &ASCII, ASCII_START)?;
        charmap_textures(textures, scale, &MAZE_WALLS, MAZE_WALLS_START)?;
        charmap_textures(textures, scale, &BULLETS, BULLETS_START)?;
        charmap_textures(textures, scale, &BRATS, BRATS_START)?;
        wide_charmap_textures(textures, scale, &FACTORIES, FACTORIES_START)?;
        wide_charmap_textures(textures, scale, &PLAYER, PLAYER_START)?;
        wide_charmap_textures(textures, scale, &RATS, RATS_START)?;
        Ok(())
    }
}

fn clear_charmap_textures(
    textures: &mut [Texture],
    scale: usize,
) -> Result<()> {
    textures[ATTR_NONE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_empty(pixels, scale);
        })
        .map_err(sdl_error)?;
    textures[ATTR_REVERSE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_empty(pixels, scale);
        })
        .map_err(sdl_error)?;
    textures[ATTR_DIM as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_empty(pixels, scale);
        })
        .map_err(sdl_error)?;
    textures[(ATTR_REVERSE | ATTR_DIM) as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_empty(pixels, scale);
        })
        .map_err(sdl_error)?;
    Ok(())
}

fn charmap_textures(
    textures: &mut [Texture],
    scale: usize,
    bitmap: &[u8],
    first: u8,
) -> Result<()> {
    textures[ATTR_NONE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels(pixels, bitmap, first, ATTR_NONE, scale);
        })
        .map_err(sdl_error)?;
    textures[ATTR_REVERSE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels(pixels, bitmap, first, ATTR_REVERSE, scale);
        })
        .map_err(sdl_error)?;
    textures[ATTR_DIM as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels(pixels, bitmap, first, ATTR_DIM, scale);
        })
        .map_err(sdl_error)?;
    textures[(ATTR_REVERSE | ATTR_DIM) as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels(pixels, bitmap, first, ATTR_REVERSE | ATTR_DIM, scale);
        })
        .map_err(sdl_error)?;
    Ok(())
}

fn set_pixels(
    pixels: &mut [u8],
    bitmap: &[u8],
    first: u8,
    attrs: u8,
    scale: usize,
) {
    assert_eq!(bitmap.len() % CHAR_CELL_HEIGHT, 0);
    assert_eq!(
        pixels.len(),
        FONT_SIZE as usize
            * BYTES_PER_PIXEL
            * (CHAR_CELL_WIDTH * scale)
            * (CHAR_CELL_HEIGHT * scale)
    );
    let offset: usize = first as usize
        * BYTES_PER_PIXEL
        * (CHAR_CELL_WIDTH * scale)
        * (CHAR_CELL_HEIGHT * scale);
    const BACKGROUND: u32 = 0x252919;
    const FOREGROUND_NORMAL: u32 = 0x5bffff;
    const FOREGROUND_DIM: u32 = 0x2d8080;
    let (fg, bg) = match (attrs & ATTR_REVERSE != 0, attrs & ATTR_DIM != 0) {
        (true, true) => (BACKGROUND, FOREGROUND_DIM),
        (true, false) => (BACKGROUND, FOREGROUND_NORMAL),
        (false, true) => (FOREGROUND_DIM, BACKGROUND),
        (false, false) => (FOREGROUND_NORMAL, BACKGROUND),
    };
    match scale {
        1 => set_pixels_1x1(pixels, bitmap, offset, fg, bg),
        2 => set_pixels_2x2(pixels, bitmap, offset, fg, bg),
        3 => set_pixels_3x3(pixels, bitmap, offset, fg, bg),
        4 => set_pixels_4x4(pixels, bitmap, offset, fg, bg),
        _ => {}
    };
}

fn set_pixels_1x1(
    pixels: &mut [u8],
    bitmap: &[u8],
    mut offset: usize,
    fg: u32,
    bg: u32,
) {
    for byte in bitmap {
        let mut mask = 0x80;
        while mask != 0 {
            let color = if byte & mask != 0 { fg } else { bg };
            pixels[offset] = ((color >> 16) & 0xff) as u8;
            pixels[offset + 1] = ((color >> 8) & 0xff) as u8;
            pixels[offset + 2] = (color & 0xff) as u8;
            offset += BYTES_PER_PIXEL;
            mask >>= 1;
        }
    }
}

fn set_pixels_2x2(
    pixels: &mut [u8],
    bitmap: &[u8],
    mut offset: usize,
    fg: u32,
    bg: u32,
) {
    for byte in bitmap {
        for r in 0..2 {
            let mut mask = 0x80;
            while mask != 0 {
                for c in 0..2 {
                    let color = if byte & mask != 0 {
                        foreground(fg, r + c == 2)
                    } else {
                        bg
                    };
                    pixels[offset] = ((color >> 16) & 0xff) as u8;
                    pixels[offset + 1] = ((color >> 8) & 0xff) as u8;
                    pixels[offset + 2] = (color & 0xff) as u8;
                    offset += BYTES_PER_PIXEL;
                }
                mask >>= 1;
            }
        }
    }
}

fn set_pixels_3x3(
    pixels: &mut [u8],
    bitmap: &[u8],
    mut offset: usize,
    fg: u32,
    bg: u32,
) {
    for byte in bitmap {
        for r in 0..3 {
            let mut mask = 0x80;
            while mask != 0 {
                for c in 0..3 {
                    let color = if byte & mask != 0 {
                        foreground(fg, r == 2 || c == 2)
                    } else {
                        bg
                    };
                    pixels[offset] = ((color >> 16) & 0xff) as u8;
                    pixels[offset + 1] = ((color >> 8) & 0xff) as u8;
                    pixels[offset + 2] = (color & 0xff) as u8;
                    offset += BYTES_PER_PIXEL;
                }
                mask >>= 1;
            }
        }
    }
}

fn set_pixels_4x4(
    pixels: &mut [u8],
    bitmap: &[u8],
    mut offset: usize,
    fg: u32,
    bg: u32,
) {
    for byte in bitmap {
        for r in 0..4 {
            let mut mask = 0x80;
            while mask != 0 {
                for c in 0..4 {
                    let color = if byte & mask != 0 {
                        foreground(fg, r == 0 || r == 2 || c == 0 || c == 2)
                    } else {
                        bg
                    };
                    pixels[offset] = ((color >> 16) & 0xff) as u8;
                    pixels[offset + 1] = ((color >> 8) & 0xff) as u8;
                    pixels[offset + 2] = (color & 0xff) as u8;
                    offset += BYTES_PER_PIXEL;
                }
                mask >>= 1;
            }
        }
    }
}

fn foreground(color: u32, predicate: bool) -> u32 {
    if predicate {
        let red = (((color >> 16) & 0xff) * 2) / 4;
        let green = (((color >> 8) & 0xff) * 2) / 4;
        let blue = ((color & 0xff) * 2) / 4;
        (red << 16) | (green << 8) | blue
    } else {
        color
    }
}

fn wide_charmap_textures(
    textures: &mut [Texture],
    scale: usize,
    bitmap: &[u16],
    first: u8,
) -> Result<()> {
    textures[ATTR_NONE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_wide(pixels, bitmap, first, ATTR_NONE, scale);
        })
        .map_err(sdl_error)?;
    textures[ATTR_REVERSE as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_wide(pixels, bitmap, first, ATTR_REVERSE, scale);
        })
        .map_err(sdl_error)?;
    textures[ATTR_DIM as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_wide(pixels, bitmap, first, ATTR_DIM, scale);
        })
        .map_err(sdl_error)?;
    textures[(ATTR_REVERSE | ATTR_DIM) as usize]
        .with_lock(None, |pixels, _pitch| {
            set_pixels_wide(
                pixels,
                bitmap,
                first,
                ATTR_REVERSE | ATTR_DIM,
                scale,
            );
        })
        .map_err(sdl_error)?;
    Ok(())
}

fn set_pixels_wide(
    pixels: &mut [u8],
    mut bitmap: &[u16],
    mut first: u8,
    attrs: u8,
    scale: usize,
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
        set_pixels(pixels, &bitmap2, first, attrs, scale);
        bitmap = &bitmap[CHAR_CELL_HEIGHT * 2..];
        first += 4;
    }
}

fn set_pixels_empty(pixels: &mut [u8], scale: usize) {
    assert_eq!(
        pixels.len(),
        FONT_SIZE as usize
            * BYTES_PER_PIXEL
            * (CHAR_CELL_WIDTH * scale)
            * (CHAR_CELL_HEIGHT * scale)
    );
    let mut offset = 0;
    for _ in 0..FONT_SIZE {
        for byte in EMPTY_CHAR_CELL {
            for _ in 0..scale {
                let mut mask = 0x80;
                while mask != 0 {
                    for _ in 0..scale {
                        pixels[offset] =
                            if byte & mask != 0 { 0x80 } else { 0 };
                        pixels[offset + 1] = 0;
                        pixels[offset + 2] = 0;
                        offset += BYTES_PER_PIXEL;
                    }
                    mask >>= 1;
                }
            }
        }
    }
}
