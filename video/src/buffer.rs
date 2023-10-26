#[derive(Clone)]
pub struct Buffer {
    pub rows: usize,
    pub cols: usize,
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

    pub fn set_chattr(&mut self, row: usize, col: usize, ch: u8, attr: u8) {
        if row < self.rows && col < self.cols {
            self.characters[row * self.cols + col] = ch;
            self.attributes[row * self.cols + col] = attr;
        }
    }

    pub fn get_chattr(&self, row: usize, col: usize) -> (u8, u8) {
        if row < self.rows && col < self.cols {
            (
                self.characters[row * self.cols + col],
                self.attributes[row * self.cols + col],
            )
        } else {
            (0, 0)
        }
    }

    pub fn swap(&mut self, other: &mut Buffer) {
        self.characters.swap_with_slice(&mut other.characters);
        self.attributes.swap_with_slice(&mut other.attributes);
    }

    pub fn print<S>(&mut self, row: usize, mut col: usize, attr: u8, s: S)
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
        mut row: usize,   // starting row in source buffer
        col: usize,       // starting column in source buffer
        dst: &mut Buffer, // destination buffer
        dst_row: usize,   // first row in destination buffer
    ) {
        for dst_row in dst_row..dst.rows {
            let mut col = col;
            for dst_col in 0..dst.cols {
                dst.set_char(dst_row, dst_col, self.get_char(row, col));
                dst.set_attr(dst_row, dst_col, self.get_attr(row, col));
                col = (col + 1) % dst.cols;
            }
            row = (row + 1) % dst.rows;
        }
    }

    pub fn copy_to(&self, dst: &mut Buffer) {
        assert!(self.rows == dst.rows);
        assert!(self.cols == dst.cols);
        // dst.characters.extend_from_slice(&self.characters);
        // dst.attributes.extend_from_slice(&self.attributes);
        for (index, ch) in self.characters.iter().enumerate() {
            dst.characters[index] = *ch;
            dst.attributes[index] = self.attributes[index];
        }
    }
}
