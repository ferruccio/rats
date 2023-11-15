use super::{dir, Dimensions, Direction};
use std::cmp::min;
use video::{Pos, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: Pos,
    pub col: Pos,
}

impl Position {
    pub fn left(&self, dims: Dimensions) -> Position {
        Position {
            row: self.row,
            col: if self.col == 0 {
                dims.cols - 1
            } else {
                self.col - 1
            },
        }
    }

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

    pub fn right(&self, dims: Dimensions) -> Position {
        Position {
            row: self.row,
            col: if self.col < dims.cols - 1 {
                self.col + 1
            } else {
                0
            },
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

    pub fn up(&self, dims: Dimensions) -> Position {
        Position {
            row: if self.row > 0 {
                self.row - 1
            } else {
                dims.rows - 1
            },
            col: self.col,
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

    pub fn down(&self, dims: Dimensions) -> Position {
        Position {
            row: if self.row < dims.rows - 1 {
                self.row + 1
            } else {
                0
            },
            col: self.col,
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

    pub fn direction_to(&self, pos: Position, dims: Dimensions) -> Direction {
        let dist_up = self.up(dims).distance_squared_to(pos, dims);
        let dist_down = self.down(dims).distance_squared_to(pos, dims);
        let dist_left = self.left(dims).distance_squared_to(pos, dims);
        let dist_right = self.right(dims).distance_squared_to(pos, dims);

        if dist_up < dist_down && dist_up < dist_left && dist_up < dist_right {
            dir::UP
        } else if dist_down < dist_left && dist_down < dist_right {
            dir::DOWN
        } else if dist_left < dist_right {
            dir::LEFT
        } else {
            dir::RIGHT
        }
    }

    // square of the distance between two points on a torus
    pub fn distance_squared_to(&self, pos: Position, dims: Dimensions) -> Size {
        let x1 = self.col as i32;
        let x2 = pos.col as i32;
        let y1 = self.row as i32;
        let y2 = pos.row as i32;
        let w = dims.cols as i32;
        let h = dims.rows as i32;
        // min(|x1 - x2|, w - |x1 - x2|)^2 + min(|y1 - y2|, h - |y1 - y2|)^2
        let mx = min((x1 - x2).abs(), w - (x1 - x2).abs());
        let my = min((y1 - y2).abs(), h - (y1 - y2).abs());
        (mx * mx + my * my) as Size
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{},c{}", self.row, self.col)
    }
}
