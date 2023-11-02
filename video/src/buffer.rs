use crate::{Size, SizeWrapping};

#[derive(Clone)]
pub struct Buffer {
    pub rows: Size,
    pub cols: Size,
    pub characters: Vec<u8>,
    pub attributes: Vec<u8>,
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("rows", &self.rows)
            .field("cols", &self.cols)
            .finish()
    }
}

pub const ATTR_NONE: u8 = 0x00;
pub const ATTR_REVERSE: u8 = 0x01;
pub const ATTR_DIM: u8 = 0x02;

pub const ATTR_MASK: u8 = 0x03;
pub const ATTR_COMBOS: usize = 4;

impl Buffer {
    pub fn new(rows: Size, cols: Size) -> Buffer {
        let mut buffer = Buffer {
            rows,
            cols,
            characters: vec![0; (rows * cols) as usize],
            attributes: vec![0; (rows * cols) as usize],
        };
        buffer.clear();
        buffer
    }

    pub fn clear(&mut self) {
        self.characters.fill(b' ');
        self.attributes.fill(ATTR_NONE);
    }

    pub fn set_char(&mut self, row: Size, col: Size, ch: u8) {
        if row < self.rows && col < self.cols {
            self.characters[(row * self.cols + col) as usize] = ch;
        }
    }

    pub fn get_char(&self, row: Size, col: Size) -> u8 {
        if row < self.rows && col < self.cols {
            self.characters[(row * self.cols + col) as usize]
        } else {
            0
        }
    }

    pub fn set_attr(&mut self, row: Size, col: Size, ch: u8) {
        if row < self.rows && col < self.cols {
            self.attributes[(row * self.cols + col) as usize] = ch & ATTR_MASK;
        }
    }

    pub fn get_attr(&self, row: Size, col: Size) -> u8 {
        if row < self.rows && col < self.cols {
            self.attributes[(row * self.cols + col) as usize]
        } else {
            0
        }
    }

    pub fn set_chattr(&mut self, row: Size, col: Size, ch: u8, attr: u8) {
        if row < self.rows && col < self.cols {
            self.set_ca(row, col, ch, attr);
        }
    }

    pub fn set_quad(&mut self, row1: Size, col1: Size, ch: u8, attr: u8) {
        if row1 < self.rows && col1 < self.cols {
            let row2 = row1.inc(self.rows);
            let col2 = col1.inc(self.cols);
            self.set_ca(row1, col1, ch, attr);
            self.set_ca(row1, col2, ch + 1, attr);
            self.set_ca(row2, col1, ch + 2, attr);
            self.set_ca(row2, col2, ch + 3, attr);
        }
    }

    fn set_ca(&mut self, row: Size, col: Size, ch: u8, attr: u8) {
        self.characters[(row * self.cols + col) as usize] = ch;
        self.attributes[(row * self.cols + col) as usize] = attr;
    }

    pub fn get_chattr(&self, row: Size, col: Size) -> (u8, u8) {
        if row < self.rows && col < self.cols {
            (
                self.characters[(row * self.cols + col) as usize],
                self.attributes[(row * self.cols + col) as usize],
            )
        } else {
            (0, 0)
        }
    }

    pub fn swap(&mut self, other: &mut Buffer) {
        self.characters.swap_with_slice(&mut other.characters);
        self.attributes.swap_with_slice(&mut other.attributes);
    }

    pub fn print<S>(&mut self, row: Size, mut col: Size, attr: u8, s: S)
    where
        S: AsRef<str>,
    {
        for ch in s.as_ref().chars() {
            // assumes that only one-byte characters are used
            self.set_char(row, col, (ch as u32 & 0xff) as u8);
            self.set_attr(row, col, attr);
            col += 1;
        }
    }

    pub fn copy_buffer(
        &self,
        mut src_row: Size, // starting row in source buffer
        src_col: Size,     // starting column in source buffer
        dst: &mut Buffer,  // destination buffer
        dst_row: Size,     // first row in destination buffer
    ) {
        for dst_row in dst_row..dst.rows {
            let mut col = src_col;
            for dst_col in 0..dst.cols {
                dst.set_char(dst_row, dst_col, self.get_char(src_row, col));
                dst.set_attr(dst_row, dst_col, self.get_attr(src_row, col));
                col = col.inc(self.cols);
            }
            src_row = src_row.inc(self.rows);
        }
    }

    pub fn copy_to(&self, dst: &mut Buffer) {
        assert!(self.rows == dst.rows && self.cols == dst.cols);
        dst.characters.copy_from_slice(&self.characters);
        dst.attributes.copy_from_slice(&self.attributes);
    }
}
