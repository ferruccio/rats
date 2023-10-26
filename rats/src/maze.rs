use video::{Buffer, ATTR_DIM, MAZE_ACROSS, MAZE_CROSS, MAZE_DOWN};

#[derive(Debug, Clone)]
pub struct Maze {
    height: usize,
    width: usize,
    pub buffer: Buffer,
}

// maze cell dimensions in characters
pub const MAZE_CELL_HEIGHT: usize = 10;
pub const MAZE_CELL_WIDTH: usize = 20;

impl Maze {
    pub fn new(height: usize, width: usize) -> Maze {
        assert!(width >= 2 && height >= 2, "invalid maze dimensions");
        let char_width = (MAZE_CELL_WIDTH + 1) * width;
        let char_height = (MAZE_CELL_HEIGHT + 1) * height;
        Maze {
            width,
            height,
            buffer: Buffer::new(char_height, char_width),
        }
    }

    #[allow(unused)]
    pub fn test_pattern(&mut self) {
        for mrow in 0..self.height {
            let row = mrow * (MAZE_CELL_HEIGHT + 1);
            for mcol in 0..self.width {
                let col = mcol * (MAZE_CELL_WIDTH + 1);
                self.buffer.set_char(row, col, MAZE_CROSS);
                for c in 1..=MAZE_CELL_WIDTH {
                    self.buffer.set_char(row, col + c, MAZE_ACROSS);
                }
                for r in 1..=MAZE_CELL_HEIGHT {
                    self.buffer.set_char(row + r, col, MAZE_DOWN)
                }
                self.buffer.print(
                    row + 4,
                    col + 4,
                    ATTR_DIM,
                    format!("r: {mrow}"),
                );
                self.buffer.print(
                    row + 5,
                    col + 4,
                    ATTR_DIM,
                    format!("c: {mcol}"),
                );
            }
        }
    }
}
