use video::{
    Chars, ATTR_NONE, PLAYER_DOWN, PLAYER_LEFT, PLAYER_RIGHT, PLAYER_UP,
};

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
pub const DIR_UP_LEFT: usize = DIR_UP | DIR_LEFT;
pub const DIR_UP_RIGHT: usize = DIR_UP | DIR_RIGHT;
pub const DIR_DOWN_LEFT: usize = DIR_DOWN | DIR_LEFT;
pub const DIR_DOWN_RIGHT: usize = DIR_DOWN | DIR_RIGHT;

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

    pub fn advance_all(&mut self, maze: &Maze, direction: Direction) {
        if self.can_advance(maze, direction) {
            self.advance(direction);
        } else {
            if direction & DIR_UP != 0 && self.can_advance(maze, DIR_UP) {
                self.advance(DIR_UP);
            }
            if direction & DIR_DOWN != 0 && self.can_advance(maze, DIR_DOWN) {
                self.advance(DIR_DOWN);
            }
            if direction & DIR_LEFT != 0 && self.can_advance(maze, DIR_LEFT) {
                self.advance(DIR_LEFT);
            }
            if direction & DIR_RIGHT != 0 && self.can_advance(maze, DIR_RIGHT) {
                self.advance(DIR_RIGHT);
            }
        }
    }

    pub fn can_advance(&self, maze: &Maze, direction: Direction) -> bool {
        let row = self.position.row;
        let col = self.position.col;
        let row_plus_1 = (row + 1) % self.position.maze_rows;
        let row_minus_1 = if row > 0 {
            row - 1
        } else {
            self.position.maze_rows
        };
        let col_plus_1 = (col + 1) % self.position.maze_cols;
        let col_minus_1 = if col > 0 {
            col - 1
        } else {
            self.position.maze_cols
        };
        match direction {
            DIR_DOWN => maze.empty(row_plus_1, col),
            DIR_DOWN_LEFT => maze.empty(row_plus_1, col_minus_1),
            DIR_DOWN_RIGHT => maze.empty(row_plus_1, col_plus_1),
            DIR_UP => maze.empty(row_minus_1, col),
            DIR_UP_LEFT => maze.empty(row_minus_1, col_minus_1),
            DIR_UP_RIGHT => maze.empty(row_minus_1, col_plus_1),
            DIR_LEFT => maze.empty(row, col_minus_1),
            DIR_RIGHT => maze.empty(row, col_plus_1),
            _ => false,
        }
    }

    pub fn render(&self, maze: &mut Maze, direction: Direction, offset: u8) {
        let row1 = self.position.row;
        let col1 = self.position.col;
        let row2 = (self.position.row + 1) % self.position.maze_rows;
        let col2 = (self.position.col + 1) % self.position.maze_cols;
        let ch = match direction {
            DIR_DOWN => PLAYER_DOWN,
            DIR_DOWN_LEFT => PLAYER_LEFT,
            DIR_DOWN_RIGHT => PLAYER_RIGHT,
            DIR_UP => PLAYER_UP,
            DIR_UP_LEFT => PLAYER_LEFT,
            DIR_UP_RIGHT => PLAYER_RIGHT,
            DIR_LEFT => PLAYER_LEFT,
            DIR_RIGHT => PLAYER_RIGHT,
            _ => PLAYER_DOWN,
        } + offset * 4;
        maze.buffer.set_chattr(row1, col1, ch, ATTR_NONE);
        maze.buffer.set_chattr(row1, col2, ch + 1, ATTR_NONE);
        maze.buffer.set_chattr(row2, col1, ch + 2, ATTR_NONE);
        maze.buffer.set_chattr(row2, col2, ch + 3, ATTR_NONE);
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
