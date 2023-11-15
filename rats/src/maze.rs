use crate::entities::Dimensions;
use knossos::maze::{HuntAndKill, OrthogonalMazeBuilder};
use rand::{distributions::Uniform, thread_rng, Rng};
use std::cell::RefCell;
use video::{
    Buffer, Pos, Size, ATTR_NONE, MAZE_ACROSS, MAZE_BOTTOM, MAZE_BOTTOM_LEFT,
    MAZE_BOTTOM_RIGHT, MAZE_BOTTOM_T, MAZE_CROSS, MAZE_DOWN, MAZE_LEFT,
    MAZE_LEFT_T, MAZE_NONE, MAZE_RIGHT, MAZE_RIGHT_T, MAZE_TOP, MAZE_TOP_LEFT,
    MAZE_TOP_RIGHT, MAZE_TOP_T, MAZE_WALLS_END, MAZE_WALLS_START,
};

#[derive(Debug, Clone)]
pub struct Maze {
    // size in maze cells
    cell_rows: Size,
    cell_cols: Size,
    // size in characters
    pub dimensions: Dimensions,
    pub buffer: Buffer,
}

thread_local! {
    pub static PRISTINE_MAZE:RefCell<Maze> = RefCell::new(Maze::new(10,10));
}

pub fn with_pristine_maze<F, T>(action: F) -> T
where
    F: Fn(&Maze) -> T,
{
    PRISTINE_MAZE.with(|maze| action(&maze.borrow()))
}

// maze cell dimensions
pub const MAZE_CELL_ROWS: Size = 9;
pub const MAZE_CELL_COLS: Size = 16;

impl Maze {
    pub fn new(cell_rows: Size, cell_cols: Size) -> Maze {
        assert!(cell_cols >= 2 && cell_rows >= 2, "invalid maze dimensions");
        let rows: Size = (MAZE_CELL_ROWS + 1) * cell_rows as Size;
        let cols: Size = (MAZE_CELL_COLS + 1) * cell_cols as Size;
        Maze {
            cell_rows,
            cell_cols,
            dimensions: Dimensions { rows, cols },
            buffer: Buffer::new(rows, cols),
        }
    }

    pub fn rows(&self) -> Size {
        self.dimensions.rows
    }

    pub fn cols(&self) -> Size {
        self.dimensions.cols
    }

    pub fn is_wall(&self, row: Pos, col: Pos) -> bool {
        let ch = self.buffer.get_char(row, col);
        (MAZE_WALLS_START..=MAZE_WALLS_END).contains(&ch)
    }

    pub fn generate(&mut self, density: usize) {
        let mut maze_grid = create_maze_grid(self.cell_rows, self.cell_cols);
        let mut rng = thread_rng();
        let distribution = Uniform::new_inclusive(1, 100);
        for cell_row in 0..self.cell_rows {
            for cell_col in 0..self.cell_cols {
                if density == 0 || rng.sample(distribution) > density {
                    maze_grid.clear(cell_row, cell_col);
                }
            }
        }
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
}

#[derive(Clone)]
pub struct Walls {
    top: bool,
    left: bool,
}

pub struct MazeGrid {
    rows: Size,
    cols: Size,
    grid: Vec<Walls>,
}

impl MazeGrid {
    pub fn new(rows: Size, cols: Size) -> MazeGrid {
        MazeGrid {
            rows,
            cols,
            grid: vec![
                Walls {
                    top: false,
                    left: false
                };
                (rows * cols) as usize
            ],
        }
    }

    pub fn get(&self, row: Pos, col: Pos) -> Walls {
        if row < self.rows && col < self.cols {
            self.grid[(row * self.cols + col) as usize].clone()
        } else {
            Walls {
                top: false,
                left: false,
            }
        }
    }

    pub fn set(&mut self, row: Pos, col: Pos, walls: Walls) {
        if row < self.rows && col < self.cols {
            self.grid[(row * self.cols + col) as usize] = walls;
        }
    }

    pub fn clear(&mut self, row: Pos, col: Pos) {
        if row < self.rows && col < self.cols {
            self.grid[(row * self.cols + col) as usize] = Walls {
                top: false,
                left: false,
            };
        }
    }

    fn joiner(&self, row: Pos, col: Pos) -> u8 {
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

fn create_maze_grid(rows: Size, cols: Size) -> MazeGrid {
    let mut maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(HuntAndKill::new()))
        .height(rows as usize)
        .width(cols as usize)
        .build();
    let grid = maze.get_grid_mut();
    let mut maze_grid = MazeGrid::new(rows, cols);
    const TOP: u8 = 0b0001;
    const LEFT: u8 = 0b1000;
    for row in 0..rows {
        for col in 0..cols {
            let bits = grid.get_cell((col as usize, row as usize)).bits();
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
