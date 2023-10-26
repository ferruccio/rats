use video::ATTR_REVERSE;

use crate::maze::Maze;

#[derive(Debug)]
pub struct Player {
    // player position within maze
    row: usize,
    col: usize,
    // maze dimensions
    maze_rows: usize,
    maze_cols: usize,
}

pub type Direction = usize;
pub const DIR_NONE: Direction = 0x00;
pub const DIR_UP: Direction = 0x01;
pub const DIR_DOWN: Direction = 0x02;
pub const DIR_LEFT: Direction = 0x04;
pub const DIR_RIGHT: Direction = 0x08;

impl Player {
    pub fn new(maze: &Maze) -> Player {
        let mut player = Player {
            row: 0,
            col: 0,
            maze_rows: maze.buffer.rows,
            maze_cols: maze.buffer.cols,
        };
        player.move_down(maze.buffer.rows / 2);
        player.move_right(maze.buffer.cols / 2);
        player
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn advance(&mut self, direction: Direction) {
        if direction & DIR_UP != 0 {
            self.move_up(1);
        }
        if direction & DIR_DOWN != 0 {
            self.move_down(1);
        }
        if direction & DIR_LEFT != 0 {
            self.move_left(1);
        }
        if direction & DIR_RIGHT != 0 {
            self.move_right(1);
        }
    }

    pub fn render(&self, maze: &mut Maze) {
        maze.buffer
            .set_chattr(self.row, self.col, b'/', ATTR_REVERSE);
        maze.buffer
            .set_chattr(self.row, self.col + 1, b'\\', ATTR_REVERSE);
        maze.buffer
            .set_chattr(self.row + 1, self.col, b'\\', ATTR_REVERSE);
        maze.buffer
            .set_chattr(self.row + 1, self.col + 1, b'/', ATTR_REVERSE);
    }
}

// private functions
impl Player {
    fn move_left(&mut self, mut steps: usize) {
        while steps > 0 {
            self.col = if self.col == 0 {
                self.maze_cols - 1
            } else {
                self.col - 1
            };
            steps -= 1;
        }
    }

    fn move_right(&mut self, mut steps: usize) {
        while steps > 0 {
            self.col = (self.col + 1) % self.maze_cols;
            steps -= 1;
        }
    }

    fn move_up(&mut self, mut steps: usize) {
        while steps > 0 {
            self.row = if self.row == 0 {
                self.maze_rows - 1
            } else {
                self.row - 1
            };
            steps -= 1;
        }
    }

    fn move_down(&mut self, mut steps: usize) {
        while steps > 0 {
            self.row = (self.row + 1) % self.maze_rows;
            steps -= 1;
        }
    }
}
