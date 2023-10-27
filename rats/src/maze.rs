use video::{Buffer, Chars, ATTR_DIM, MAZE_ACROSS, MAZE_CROSS, MAZE_DOWN};

#[derive(Debug)]
pub struct Maze {
    // size in maze cells
    cell_rows: usize,
    cell_cols: usize,
    // size in characters
    rows: Chars,
    cols: Chars,
    pub buffer: Buffer,
}

// maze cell dimensions
pub const MAZE_CELL_ROWS: Chars = 10;
pub const MAZE_CELL_COLS: Chars = 20;

impl Maze {
    pub fn new(cell_rows: usize, cell_cols: usize) -> Maze {
        assert!(cell_cols >= 2 && cell_rows >= 2, "invalid maze dimensions");
        let rows: Chars = (MAZE_CELL_ROWS + 1) * cell_rows;
        let cols: Chars = (MAZE_CELL_COLS + 1) * cell_cols;
        Maze {
            cell_rows,
            cell_cols,
            rows,
            cols,
            buffer: Buffer::new(rows, cols),
        }
    }

    pub fn rows(&self) -> Chars {
        self.rows
    }

    pub fn cols(&self) -> Chars {
        self.cols
    }

    #[allow(unused)]
    pub fn test_pattern(&mut self) {
        for cell_row in 0..self.cell_rows {
            let row = cell_row * (MAZE_CELL_ROWS + 1);
            for cell_col in 0..self.cell_cols {
                let col = cell_col * (MAZE_CELL_COLS + 1);
                self.buffer.set_char(row, col, MAZE_CROSS);
                for col_index in 1..=MAZE_CELL_COLS {
                    self.buffer.set_char(row, col + col_index, MAZE_ACROSS);
                }
                for row_index in 1..=MAZE_CELL_ROWS {
                    self.buffer.set_char(row + row_index, col, MAZE_DOWN)
                }
                self.buffer.print(
                    row + 4,
                    col + 4,
                    ATTR_DIM,
                    format!("r: {cell_row}"),
                );
                self.buffer.print(
                    row + 5,
                    col + 4,
                    ATTR_DIM,
                    format!("c: {cell_col}"),
                );
            }
        }
    }
}
