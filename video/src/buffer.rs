pub struct Buffer {
    pub rows: usize,
    pub cols: usize,
    pub characters: Vec<u8>,
    pub attributes: Vec<u8>,
}

pub const ATTR_NONE: u8 = 0x00;
pub const ATTR_REVERSE: u8 = 0x01;
pub const ATTR_DIM: u8 = 0x02;

pub const ATTR_MASK: u8 = 0x03;
pub const ATTR_COMBOS: usize = 4;

impl Buffer {
    pub fn new(rows: usize, cols: usize) -> Buffer {
        let mut buffer = Buffer {
            rows,
            cols,
            characters: vec![0; rows * cols],
            attributes: vec![0; rows * cols],
        };
        buffer.clear();
        buffer
    }

    pub fn clear(&mut self) {
        self.characters.fill(b' ');
        self.attributes.fill(ATTR_NONE);
    }

    pub fn set_char(&mut self, row: usize, col: usize, ch: u8) {
        if row < self.rows && col < self.cols {
            self.characters[row * self.cols + col] = ch;
        }
    }

    pub fn get_char(&self, row: usize, col: usize) -> u8 {
        if row < self.rows && col < self.cols {
            self.characters[row * self.cols + col]
        } else {
            0
        }
    }

    pub fn set_attr(&mut self, row: usize, col: usize, ch: u8) {
        if row < self.rows && col < self.cols {
            self.attributes[row * self.cols + col] = ch & ATTR_MASK;
        }
    }

    pub fn get_attr(&self, row: usize, col: usize) -> u8 {
        if row < self.rows && col < self.cols {
            self.attributes[row * self.cols + col]
        } else {
            0
        }
    }

    pub fn swap(&mut self, other: &mut Buffer) {
        self.characters.swap_with_slice(&mut other.characters);
        self.attributes.swap_with_slice(&mut other.attributes);
    }
}
