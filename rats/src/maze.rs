use knossos::maze::{HuntAndKill, OrthogonalMazeBuilder};
use video::{
    Buffer, Chars, ATTR_DIM, ATTR_NONE, MAZE_ACROSS, MAZE_BOTTOM,
    MAZE_BOTTOM_LEFT, MAZE_BOTTOM_RIGHT, MAZE_BOTTOM_T, MAZE_CROSS, MAZE_DOWN,
    MAZE_LEFT, MAZE_LEFT_T, MAZE_NONE, MAZE_RIGHT, MAZE_RIGHT_T, MAZE_TOP,
    MAZE_TOP_LEFT, MAZE_TOP_RIGHT, MAZE_TOP_T, MAZE_WALLS_END,
    MAZE_WALLS_START,
};

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

    pub fn empty(&self, row: Chars, col: Chars) -> bool {
        !is_wall_char(self.buffer.get_char(row, col))
            && !is_wall_char(self.buffer.get_char(row, col + 1))
            && !is_wall_char(self.buffer.get_char(row + 1, col))
            && !is_wall_char(self.buffer.get_char(row + 1, col + 1))
    }

    pub fn generate(&mut self) {
        let maze_grid = create_maze_grid(self.cell_rows, self.cell_cols);
        for cell_row in 0..self.cell_rows {
            let row = cell_row * (MAZE_CELL_ROWS + 1);
            for cell_col in 0..self.cell_cols {
                let col = cell_col * (MAZE_CELL_COLS + 1);
                let walls = maze_grid.get(cell_row, cell_col);
                self.buffer.set_chattr(
                    row,
                    col,
                    maze_grid.joiner(cell_row, cell_col),
                    ATTR_NONE,
                );
                if walls.top {
                    for col_index in 1..=MAZE_CELL_COLS {
                        self.buffer.set_chattr(
                            row,
                            col + col_index,
                            MAZE_ACROSS,
                            ATTR_NONE,
                        );
                    }
                }
                if walls.left {
                    for row_index in 1..=MAZE_CELL_ROWS {
                        self.buffer.set_chattr(
                            row + row_index,
                            col,
                            MAZE_DOWN,
                            ATTR_NONE,
                        );
                    }
                }
            }
        }
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

#[derive(Clone)]
pub struct Walls {
    top: bool,
    left: bool,
}

pub struct MazeGrid {
    rows: usize,
    cols: usize,
    grid: Vec<Walls>,
}

impl MazeGrid {
    pub fn new(rows: usize, cols: usize) -> MazeGrid {
        MazeGrid {
            rows,
            cols,
            grid: vec![
                Walls {
                    top: false,
                    left: false
                };
                rows * cols
            ],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Walls {
        if row < self.rows && col < self.cols {
            self.grid[row * self.cols + col].clone()
        } else {
            Walls {
                top: false,
                left: false,
            }
        }
    }

    pub fn set(&mut self, row: usize, col: usize, walls: Walls) {
        if row < self.rows && col < self.cols {
            self.grid[row * self.cols + col] = walls;
        }
    }

    #[allow(unused)]
    pub fn clear(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            self.grid[row * self.cols + col] = Walls {
                top: false,
                left: false,
            };
        }
    }

    fn joiner(&self, row: usize, col: usize) -> u8 {
        let row_1 = if row == 0 { self.rows - 1 } else { row - 1 };
        let col_1 = if col == 0 { self.cols - 1 } else { col - 1 };
        let mut wall_index = 0;
        if self.get(row_1, col).left {
            wall_index |= 0b_1000;
        }
        if self.get(row, col).top {
            wall_index |= 0b_0100;
        }
        if self.get(row, col).left {
            wall_index |= 0b_0010;
        }
        if self.get(row, col_1).top {
            wall_index |= 0b_0001;
        }
        let wall_chars: [u8; 16] = [
            /* 0000 */ MAZE_NONE,
            /* 0001 */ MAZE_LEFT,
            /* 0010 */ MAZE_BOTTOM,
            /* 0011 */ MAZE_TOP_RIGHT,
            /* 0100 */ MAZE_RIGHT,
            /* 0101 */ MAZE_ACROSS,
            /* 0110 */ MAZE_TOP_LEFT,
            /* 0111 */ MAZE_TOP_T,
            /* 1000 */ MAZE_TOP,
            /* 1001 */ MAZE_BOTTOM_LEFT,
            /* 1010 */ MAZE_DOWN,
            /* 1011 */ MAZE_RIGHT_T,
            /* 1100 */ MAZE_BOTTOM_RIGHT,
            /* 1101 */ MAZE_BOTTOM_T,
            /* 1110 */ MAZE_LEFT_T,
            /* 1111 */ MAZE_CROSS,
        ];
        wall_chars[wall_index]
    }
}

fn create_maze_grid(rows: usize, cols: usize) -> MazeGrid {
    let mut maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(HuntAndKill::new()))
        .height(rows)
        .width(cols)
        .build();
    let grid = maze.get_grid_mut();
    let mut maze_grid = MazeGrid::new(rows, cols);
    const TOP: u8 = 0b0001;
    const LEFT: u8 = 0b1000;
    for row in 0..rows {
        for col in 0..cols {
            let bits = grid.get_cell((col, row)).bits();
            maze_grid.set(
                row,
                col,
                Walls {
                    top: bits & TOP == 0,
                    left: bits & LEFT == 0,
                },
            )
        }
    }
    maze_grid
}

fn is_wall_char(ch: u8) -> bool {
    ch >= MAZE_WALLS_START && ch <= MAZE_WALLS_END
}
