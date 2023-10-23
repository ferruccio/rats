pub struct Buffer {
    pub rows: usize,
    pub cols: usize,
    pub buffer: Vec<u8>,
}

impl Buffer {
    pub fn new(rows: usize, cols: usize) -> Buffer {
        let mut buffer = Buffer {
            rows,
            cols,
            buffer: vec![0; rows * cols],
        };
        buffer.clear();
        buffer
    }

    pub fn clear(&mut self) {
        self.buffer.fill(b' ');
    }

    pub fn set(&mut self, row: usize, col: usize, ch: u8) {
        if row < self.rows && col < self.cols {
            self.buffer[row * self.cols + col] = ch;
        }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        if row < self.rows && col < self.cols {
            self.buffer[row * self.cols + col]
        } else {
            0
        }
    }

    pub fn swap(&mut self, other: &mut Buffer) {
        self.buffer.swap_with_slice(&mut other.buffer);
    }
}
