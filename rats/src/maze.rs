use knossos::maze::{HuntAndKill, OrthogonalMaze, OrthogonalMazeBuilder};
use video::{
    Buffer, Chars, ATTR_DIM, ATTR_NONE, MAZE_ACROSS, MAZE_BOTTOM,
    MAZE_BOTTOM_LEFT, MAZE_BOTTOM_RIGHT, MAZE_BOTTOM_T, MAZE_CROSS, MAZE_DOWN,
    MAZE_LEFT, MAZE_LEFT_T, MAZE_NONE, MAZE_RIGHT, MAZE_RIGHT_T, MAZE_TOP,
    MAZE_TOP_LEFT, MAZE_TOP_RIGHT, MAZE_TOP_T,
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

    pub fn generate(&mut self, legend: bool) {
        let mut gen = MazeGenerator::new(self.cell_rows, self.cell_cols);
        for cell_row in 0..self.cell_rows {
            let row = cell_row * (MAZE_CELL_ROWS + 1);
            for cell_col in 0..self.cell_cols {
                let col = cell_col * (MAZE_CELL_COLS + 1);
                let walls = gen.walls(cell_row, cell_col);
                self.buffer.set_chattr(
                    row,
                    col,
                    gen.joiner(cell_row, cell_col),
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
                        if legend {
                            self.buffer.print(
                                row + row_index,
                                col + 1,
                                ATTR_DIM,
                                format!("{row_index:X}"),
                            );
                        }
                    }
                }
                if legend {
                    self.buffer.print(
                        row + 4,
                        col + 6,
                        ATTR_DIM,
                        format!("r: {cell_row}"),
                    );
                    self.buffer.print(
                        row + 5,
                        col + 6,
                        ATTR_DIM,
                        format!("c: {cell_col}"),
                    );
                    self.buffer.print(
                        row + 6,
                        col + 6,
                        ATTR_DIM,
                        format!(
                            "b: {bits:04b}",
                            bits = gen.bits(row, col) & 0xf
                        ),
                    );
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

struct MazeGenerator {
    rows: usize,
    cols: usize,
    maze: OrthogonalMaze,
}

impl std::fmt::Debug for MazeGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MazeGenerator")
            .field("rows", &self.rows)
            .field("cols", &self.cols)
            .finish()
    }
}

struct Walls {
    top: bool,
    left: bool,
}

impl MazeGenerator {
    fn new(rows: usize, cols: usize) -> MazeGenerator {
        let maze = OrthogonalMazeBuilder::new()
            .algorithm(Box::new(HuntAndKill::new()))
            .height(rows)
            .width(cols)
            .build();
        MazeGenerator { rows, cols, maze }
    }

    fn walls(&mut self, row: usize, col: usize) -> Walls {
        const TOP: u8 = 0b0001;
        const LEFT: u8 = 0b1000;
        let bits = self.bits(row, col);
        Walls {
            top: bits & TOP == 0,
            left: bits & LEFT == 0,
        }
    }

    fn bits(&mut self, row: usize, col: usize) -> u8 {
        self.maze
            .get_grid_mut()
            .get_cell((col % self.cols, row % self.rows))
            .bits()
    }

    fn joiner(&mut self, row: usize, col: usize) -> u8 {
        let row_1 = if row == 0 { self.rows - 1 } else { row - 1 };
        let col_1 = if col == 0 { self.cols - 1 } else { col - 1 };
        let walls = (
            self.walls(row_1, col).left, // .0
            self.walls(row, col).top,    // .1
            self.walls(row, col).left,   // .2
            self.walls(row, col_1).top,  // .3
        );
        match walls {
            (true, true, true, true) => MAZE_CROSS,
            (true, true, true, false) => MAZE_LEFT_T,
            (true, true, false, true) => MAZE_BOTTOM_T,
            (true, true, false, false) => MAZE_BOTTOM_RIGHT,
            (true, false, true, true) => MAZE_RIGHT_T,
            (true, false, true, false) => MAZE_DOWN,
            (true, false, false, true) => MAZE_BOTTOM_LEFT,
            (false, true, true, true) => MAZE_TOP_T,
            (false, true, true, false) => MAZE_TOP_LEFT,
            (false, true, false, true) => MAZE_ACROSS,
            (false, false, true, true) => MAZE_TOP_RIGHT,
            (true, false, false, false) => MAZE_TOP,
            (false, true, false, false) => MAZE_RIGHT,
            (false, false, true, false) => MAZE_BOTTOM,
            (false, false, false, true) => MAZE_LEFT,
            (false, false, false, false) => MAZE_NONE,
        }
    }
}
