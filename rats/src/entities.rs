use std::fmt::Display;
use video::{Pos, Size};

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    Rat(Rat),
    BabyRat(BabyRat),
    RatFactory(RatFactory),
    Bullet(Bullet),
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub updated: u32,
    pub pos: Position,
    pub dir: Direction,
    pub stop_dir: Direction,
    pub state: State,
    pub cycle: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Rat {
    pub updated: u32,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
    pub cycle: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct BabyRat {
    pub updated: u32,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
    pub cycle: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct RatFactory {
    pub updated: u32,
    pub pos: Position,
    pub state: State,
    pub cycle: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub updated: u32,
    pub pos: Position,
    pub dir: Direction,
    pub state: State,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: Pos,
    pub col: Pos,
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub rows: Size,
    pub cols: Size,
}

// entities list index
pub type Index = usize;

impl Position {
    pub fn move_left(&mut self, mut steps: Size, dim: Dimensions) {
        while steps > 0 {
            self.col = if self.col == 0 {
                dim.cols - 1
            } else {
                self.col - 1
            };
            steps -= 1;
        }
    }

    pub fn move_right(&mut self, mut steps: Size, dim: Dimensions) {
        while steps > 0 {
            self.col = if self.col < dim.cols - 1 {
                self.col + 1
            } else {
                0
            };
            steps -= 1;
        }
    }

    pub fn move_up(&mut self, mut steps: Size, dim: Dimensions) {
        while steps > 0 {
            self.row = if self.row > 0 {
                self.row - 1
            } else {
                dim.rows - 1
            };
            steps -= 1;
        }
    }

    pub fn move_down(&mut self, mut steps: Size, dim: Dimensions) {
        while steps > 0 {
            self.row = if self.row < dim.rows - 1 {
                self.row + 1
            } else {
                0
            };
            steps -= 1;
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.row, self.col)
    }
}

pub type Direction = u8;
pub const DIR_NONE: Direction = 0x00;
pub const DIR_UP: Direction = 0x01;
pub const DIR_DOWN: Direction = 0x02;
pub const DIR_LEFT: Direction = 0x04;
pub const DIR_RIGHT: Direction = 0x08;
pub const DIR_UP_LEFT: Direction = DIR_UP | DIR_LEFT;
pub const DIR_UP_RIGHT: Direction = DIR_UP | DIR_RIGHT;
pub const DIR_DOWN_LEFT: Direction = DIR_DOWN | DIR_LEFT;
pub const DIR_DOWN_RIGHT: Direction = DIR_DOWN | DIR_RIGHT;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Alive,
    Exploding,
    Dead,
}

pub type EntityList = Vec<Entity>;
