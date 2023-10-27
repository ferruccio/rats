use video::{Chars, ATTR_REVERSE};

use crate::maze::Maze;

#[derive(Debug)]
pub struct Player {
    position: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub row: Chars,
    pub col: Chars,
    pub maze_rows: Chars,
    pub maze_cols: Chars,
}

pub type Direction = usize;
pub const DIR_NONE: Direction = 0x00;
pub const DIR_UP: Direction = 0x01;
pub const DIR_DOWN: Direction = 0x02;
pub const DIR_LEFT: Direction = 0x04;
pub const DIR_RIGHT: Direction = 0x08;

impl Player {
    pub fn new(maze: &Maze) -> Player {
        Player {
            position: Position {
                row: 0,
                col: 0,
                maze_rows: maze.rows(),
                maze_cols: maze.cols(),
            },
        }
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }

    pub fn advance(&mut self, direction: Direction) {
        if direction & DIR_UP != 0 {
            self.position.move_up(1);
        }
        if direction & DIR_DOWN != 0 {
            self.position.move_down(1);
        }
        if direction & DIR_LEFT != 0 {
            self.position.move_left(1);
        }
        if direction & DIR_RIGHT != 0 {
            self.position.move_right(1);
        }
    }

    pub fn render(&self, maze: &mut Maze) {
        let row1 = self.position.row;
        let col1 = self.position.col;
        let row2 = (self.position.row + 1) % self.position.maze_rows;
        let col2 = (self.position.col + 1) % self.position.maze_cols;
        maze.buffer.set_chattr(row1, col1, b'/', ATTR_REVERSE);
        maze.buffer.set_chattr(row1, col2, b'\\', ATTR_REVERSE);
        maze.buffer.set_chattr(row2, col1, b'\\', ATTR_REVERSE);
        maze.buffer.set_chattr(row2, col2, b'/', ATTR_REVERSE);
    }
}

impl Position {
    pub fn move_left(&mut self, mut steps: usize) {
        while steps > 0 {
            self.col = if self.col == 0 {
                self.maze_cols - 1
            } else {
                self.col - 1
            };
            steps -= 1;
        }
    }

    pub fn move_right(&mut self, mut steps: usize) {
        while steps > 0 {
            self.col = (self.col + 1) % self.maze_cols;
            steps -= 1;
        }
    }

    pub fn move_up(&mut self, mut steps: usize) {
        while steps > 0 {
            self.row = if self.row == 0 {
                self.maze_rows - 1
            } else {
                self.row - 1
            };
            steps -= 1;
        }
    }

    pub fn move_down(&mut self, mut steps: usize) {
        while steps > 0 {
            self.row = (self.row + 1) % self.maze_rows;
            steps -= 1;
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r={row} c={col}", row = self.row, col = self.col)
    }
}
