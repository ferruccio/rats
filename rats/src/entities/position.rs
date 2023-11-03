use super::{dir, Dimensions, Direction};
use video::{Pos, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: Pos,
    pub col: Pos,
}

impl Position {
    pub fn move_left(&mut self, mut steps: Size, dims: Dimensions) {
        while steps > 0 {
            self.col = if self.col == 0 {
                dims.cols - 1
            } else {
                self.col - 1
            };
            steps -= 1;
        }
    }

    pub fn move_right(&mut self, mut steps: Size, dims: Dimensions) {
        while steps > 0 {
            self.col = if self.col < dims.cols - 1 {
                self.col + 1
            } else {
                0
            };
            steps -= 1;
        }
    }

    pub fn move_up(&mut self, mut steps: Size, dims: Dimensions) {
        while steps > 0 {
            self.row = if self.row > 0 {
                self.row - 1
            } else {
                dims.rows - 1
            };
            steps -= 1;
        }
    }

    pub fn move_down(&mut self, mut steps: Size, dims: Dimensions) {
        while steps > 0 {
            self.row = if self.row < dims.rows - 1 {
                self.row + 1
            } else {
                0
            };
            steps -= 1;
        }
    }

    pub fn advance(mut self, dir: Direction, dims: Dimensions) -> Self {
        if dir & dir::UP != 0 {
            self.move_up(1, dims);
        }
        if dir & dir::DOWN != 0 {
            self.move_down(1, dims);
        }
        if dir & dir::LEFT != 0 {
            self.move_left(1, dims);
        }
        if dir & dir::RIGHT != 0 {
            self.move_right(1, dims);
        }
        self
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.row, self.col)
    }
}
